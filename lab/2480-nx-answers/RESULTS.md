# Row 2480-NX: all eleven names, ten holding one unmeasured mark each, answered with no bytes before their ledgers existed; each created as one new ledger with its own bill, none refused; each then read by the report as its own ledger, told and equal to the other two reads

Run on 10 Nov 2026, 22:08:13Z to 22:27:22Z (03:38 to 03:57 IST, 11 Nov), on TallyPrime single-user edition: the Gateway on 9001, `tally.exe` file version 1.1.7.2 (`evidence/tally-exe.txt`, equal to row 2480-NC's; the Gateway shows no release number). The sheet was `lab/2480-nx-sheet` (ea84bcdf), with all 77 files equal to its `SHA256.txt` (sizes too); `LAB-COMMON.md`, `run.py`, `count20.py`, `nm.py`, `st.py`, `M-marks.xml`, `B-bills.xml` and the snapshot request byte-equal to row 2480-NC's. One book, `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic, in the synthetic-uuid registry), loaded alone, in a sitting of its own with a Tally start of its own. No Bridge build ran and no Bridge tool was called. Each request file's sha256 and size were checked against the sheet's `SHA256.txt` just before its send (`evidence/sends-utc.txt`).

Every value below is M, measured by this run, in one run. Every read was answered, each with curl exit 0 and HTTP 200; no dialog, message or prompt showed but the two of C6 (none at the backup, the loads or the shuts). Every capture after a send was the Gateway, one of three images of it, each looked at (Deviations).

## Result

**Step 3, before any ledger existed (p1 to p11).** Each of the eleven `L-lo-<id>-20260801.xml` requests was answered HTTP 200 with `CONTENT-LENGTH: 0` and 0 bytes: `nm.py` `ANSWER: NO BYTES` for each, the control included. `run.py` reported each as "empty answer"; each was recorded under the run lead's empty-answer exception (Deviations), each followed by one GET /status that answered 200 (51 B). **So no name was answered with another ledger's bills before its own ledger existed.** p12's marks were equal to s1's (ALTVCHID 68, ALTMSTID 245): the eleven reads changed nothing.

**Step 4, the set-up.** All 22 imports, each sent once, in table order, each after a marks read that passed `count20.py <the before-write option>`: every answer `CREATED` 1 and `ALTERED`, `DELETED`, `COMBINED`, `IGNORED`, `ERRORS`, `CANCELLED` and `EXCEPTIONS` each 0, with no LINEERROR, so `st.py --import` gave `IMPORT: CREATED ONE` (exit 0) on each. The invoices' `LASTVCHID` ran from 45 to 55 in table order. **No import was refused, so `rf.py` was never run and no id was skipped. Each of the ten names holding one of the marks `,` `_` `"` `+` `@` `#` `:` `%` `*` `!` was created as a new ledger with its bill.** The marks rose by one master per ledger and one voucher per invoice: 68/245 before the first write, 79/256 after the last (s6m).

**Step 5, the book-wide reads.** The snapshot, sent twice (s6sa, s6sb): 39,182 B each, byte-equal, 29 LEDGER rows (18 at s2), a row for each of the eleven, each `Sundry Debtors`, bill-wise Yes, opening 0.00, closing -1051.00. B-bills, sent twice (s6ba, s6bb): 5,112 B each, byte-equal, 11 BILLFIXED groups, one for each name, `BP/26-27/0261` to `BP/26-27/0271` in table order, each dated 1-Aug-26 and -1051.00, no group of any other party. `st.py --names`: `NAMES: EQUAL`, each ledger's name list its own name alone. **The intended state, as the sheet defines it.**

**Step 6, the report reads (r1a to r11b).** Each id has a ledger and a bill as step 6 defines it, so all eleven were read, each twice:

| n | id | name | its mark | r<n>a, r<n>b | rows | the bill | `--same` | `--outcome` | `--against` |
|---|---|---|---|---|---|---|---|---|---|
| 1 | `pl` | `BRIDGE Nx Plain` | none (the control) | 896 B each | 1 (0 On Account) | `BP/26-27/0261`, -1051.00 | EQUAL | told | EQUAL |
| 2 | `cm` | `BRIDGE Nx Ak, Am` | `,` | 896 B each | 1 (0 On Account) | `BP/26-27/0262`, -1051.00 | EQUAL | told | EQUAL |
| 3 | `us` | `BRIDGE Nx Bk_Bm` | `_` | 896 B each | 1 (0 On Account) | `BP/26-27/0263`, -1051.00 | EQUAL | told | EQUAL |
| 4 | `dq` | `BRIDGE Nx "Ck" Cm` | `"` | 896 B each | 1 (0 On Account) | `BP/26-27/0264`, -1051.00 | EQUAL | told | EQUAL |
| 5 | `ps` | `BRIDGE Nx Dk + Dm` | `+` | 896 B each | 1 (0 On Account) | `BP/26-27/0265`, -1051.00 | EQUAL | told | EQUAL |
| 6 | `at` | `BRIDGE Nx Ek @ Em` | `@` | 896 B each | 1 (0 On Account) | `BP/26-27/0266`, -1051.00 | EQUAL | told | EQUAL |
| 7 | `hs` | `BRIDGE Nx Fk #12` | `#` | 896 B each | 1 (0 On Account) | `BP/26-27/0267`, -1051.00 | EQUAL | told | EQUAL |
| 8 | `cl` | `BRIDGE Nx Gk: Gm` | `:` | 896 B each | 1 (0 On Account) | `BP/26-27/0268`, -1051.00 | EQUAL | told | EQUAL |
| 9 | `pc` | `BRIDGE Nx Hk 15% Hm` | `%` | 896 B each | 1 (0 On Account) | `BP/26-27/0269`, -1051.00 | EQUAL | told | EQUAL |
| 10 | `st` | `BRIDGE Nx Jk * Jm` | `*` | 896 B each | 1 (0 On Account) | `BP/26-27/0270`, -1051.00 | EQUAL | told | EQUAL |
| 11 | `ex` | `BRIDGE Nx Kk!` | `!` | 896 B each | 1 (0 On Account) | `BP/26-27/0271`, -1051.00 | EQUAL | told | EQUAL |

Each bill is dated 1-Aug-26, due 1-Aug-26, overdue 0. `--outcome` told: the sum of the bills (-1051.00) equals the snapshot's closing for that ledger. `--against` EQUAL: the report's rows equal that ledger's B-bills group, and the one bill is that ledger's own. **The control is told with `AGAINST: EQUAL`, so each other name is judged against it. On the sheet's four questions, for each of the ten marks, on this book and release: before the ledger existed the report answered with no bytes; Tally created the ledger and its bill from master's import shapes; the report then answered with that ledger's own bill, the two sends equal; and that answer agreed with the snapshot's closing and the book-wide group.** With rows 2480-NS and 2480-NC, that is evidence for a type that admits these ten marks too, one a name, on one book and release. It says nothing about the formula read (`CHILDOF`).

**Step 7.** m1's marks were equal to s6m's (79/256): steps 5 and 6 only read.

**Step 8, the way back.** BK was restored by C6, with only the two expected prompts. Then s9 sent every file of s2, in s2's order: every answer byte-equal to its s2 answer (`evidence/s9-against-s2.txt`), and the marks equal to s1's (68/245; `count20.py --same-row`). So the book holds again what it held at step 1.

**Timings.** Every read sent without a window capture after it returned within 1 s of its send line, wall clock, python's start included, but s2 and s9: s2's fourteen reads took 2.283 s together and s9's 2.891 s. The labels sent with a capture took 1.193 to 5.004 s, the capture included (`evidence/durations.txt`).

**What this does not say.** Whether the build's own parse accepts these answers; the formula read (`CHILDOF`); any other book, release or mark; a name holding two or more of these ten marks, or one of them beside NS's and NC's marks; names that share a stem with another ledger; the composed path through a built executable. `rf.py`'s refusal path met no real answer here, since nothing was refused.

## Deviations from the sheet

- **`python` for `python3`.** This PC has no `python3` command. Every script ran as `python -I` (Python 3.10), unchanged, from the row folder, through the helpers in `scripts/`.
- **The empty-answer exception.** `run.py` reports each empty L-lo- answer as a failed request. The run lead's word for this row (in the queue entry, noted at 22:06Z) made an empty L-lo- answer with HTTP 200 and `CONTENT-LENGTH: 0` not a stop, each followed by GET /status once. The helper `scripts/lr.sh` tested exactly that (headers status 200, `CONTENT-LENGTH: 0`, a 0-byte file) before the next request; each p answer met it (`evidence/empty-p*.txt`). No r answer was empty.
- **The run lead's word on a refused import** (the same entry) made an import that `rf.py` reads as refused, followed by an equal marks read, that id's result. `scripts/w.sh` carried that path (ROW.md step 4 (e)); no import took it.
- **The start-up company** loaded by itself when Tally started. It was shut (K: Company, Shut, Enter on its one row, no prompt) before the book was loaded, on the run lead's standing word.
- **The three Bridge processes** that ran before the row (two `bridge_mcp.exe` and `BridgeCommunication.exe`) were left running, as the sheet says; no process is ended on this PC. None was called by the row.
- **Sends run in the background, with a 300 s watch.** From step 3 on, each block of sends was started by `scripts/bg.sh`, which writes the block's output and exit to `evidence/run-<name>.out` and `.rc`, beside `scripts/wd.sh`, which watched the same block for 5 minutes and ended when it did. So no tool's time limit could end a curl that waited (the wedge rule: never end curl). Each block ended well inside the watch. s1 and s2 (before step 3), p12 and m1 (each one marks read) were sent in the foreground; each answered within 3 s.
- **Step 4, one id a command.** The sheet asks for one label a command. Each id's four labels (`s4<id>m`, `s4<id>`, `s5<id>m`, `s5<id>`) were sent by one command, `scripts/wid.sh <id>`, which runs `scripts/w.sh` for each in the recipe's order and ends at the first exit other than 0, sending nothing after it. Eleven commands, one per id, in table order; none ended early.
- **Three Gateway images, not one.** p1's and p2's captures were byte-equal to s1's step-0 capture. From p3 on, the capture after a send showed the same Gateway with Tally not the focused window: the bars drawn grey, the content the same, no dialog (looked at). A click on Tally's title bar to focus it again landed on its search box, and that capture differs from p1's only in the search box (x 578 to 1138, y 8 to 44 of the capture), drawn tinted. The three images, each looked at, are listed in `evidence/gateway-refs.txt`. `w.sh` and the step 6 loop accepted a capture after a send only if it was one of them, and ended with nothing more sent on any other. Every capture after a send was one of them: p1 and p2 and s9-gateway the first; p3 to p11, 21 of the 22 import captures and all 22 r captures the second; one import capture the third.
- **The wait between labels.** Step 3's p1 to p11, step 5's five reads and step 6's reads were each sent by a loop (`scripts/p.sh`, `scripts/s6.sh`, `scripts/r.sh`) that went on only when the previous label's helper exited 0 (or 3, bytes came back, at step 6).
- **Captures after r<n>b.** `lr.sh` captures the window after each of its sends, so each r<n>b has a capture as well as each r<n>a.
- **A process check run twice.** After the quit, the first `tasklist` call exited 1, because the shell turned its `/FI` filter into a path. It was run again with that conversion off: exit 0, no `tally.exe`. Both are in the notes.

## Steps
- Step 0 (22:08:13Z): the sheet's files checked; the book's data folder found once by name and GUID; the excluded GUID absent (`evidence/bookcheck.txt`). The backup folder made new and empty. No `tally.exe` ran.
- Tally started from its desktop shortcut (22:08:24Z). The start-up company was shut (Deviations). `BRIDGE`, space, `PILOT`, space, `LAB` typed in Select Company's filter; Enter only after a capture showed the one row left (`s1-select`).
- Gateway: the book alone; period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026; last entry 15-Sep-26. Step 0 C2 (22:09:20Z): one `tally.exe`, which alone owns 9001; GET /status status=200, bytes=51. s1: the book's GUID, ALTVCHID 68, ALTMSTID 245, as the run lead's condition asks.
- BK by C5 (capture before Accept 22:10:17Z): Data, Backup & Restore, Backup; Backspace to the path, Specify Path, `C:\lab\2480-nx\backup-1` (Tally showed `C:\lab\2480-Nx\backup-1`); the book the one company; End of List; Accept y; back at the Gateway with no message. One file of 90,051 B, at 03:40 IST, 11 Nov (`evidence/bk.txt`).
- Step 2 (s2): each N read 0 rows; B-bills no group; the snapshot 18 LEDGER rows and no row for any of the eleven; `st.py <the before option>` `FOLD: CLEAR`; `st.py --names` `NAMES: EQUAL`; the marks 68/245 (C5's marks read after the backup).
- Step 3 (p1 to p11 22:11:07Z to 22:11:51Z, p12 22:15:01Z), step 4 (22:15:11Z to 22:19:20Z), step 5 (22:20:19Z to 22:20:22Z), step 6 (22:21:05Z to 22:21:58Z) and step 7 as in Result, each in the notes.
- Step 8 C6 (prep 22:22:34Z: the BK re-hashed equal, its folder holding that file alone, `restore-plan.txt` written, the data folder listed): the book shut by K: Company, Shut, Enter, no prompt; Data, Backup & Restore, Restore; Restore Data Path Tally's data folder; Specify Path, the backup-1 folder; the list showed the book, its number and 11-Nov-26 (3:40 am), as the plan; the book selected alone; Accept ? y (22:24:10Z); the folder-exists question with the book's number (22:24:27Z), r Replace; back at Select Company with no other prompt. The data folder listed again: seven of Tally's own files present before the restore were not there after it (`Index.1800`, `CmpSchMetaData.1800`, `TCMPSCHACCESS.TSF`, `TCMPSCHMESSAGE.TSF`, `TCMPSCHSTATE.TSF`, `TCMPSCHUPDATE.TSF`, `TINTMSG.TSF`); row 2480-NC's restore had left out only `Index.1800`. The book loaded alone by the name filter; Gateway captured, byte-equal to p1's, date and period unchanged; then s9, equal to s2 throughout.
- Step 9, C10 (22:27:00Z): the book shut by K: Company, Shut, Enter on its one row, no prompt; Select Company with none loaded; one `tally.exe` owning 9001; GET /status status=200, bytes=51. No request after the shut. Tally then quit by its own Quit, no prompt; at 22:27:22Z no `tally.exe` ran and nothing listened on 9001. The BK, listed again, one file, its sha256 equal to `evidence/bk.txt`.

## Files
- `answers/`: every answer and its headers file, whole, byte for byte (`kept-local.txt` lists any that stay on the lab PC).
- `logs/`: `run.py`'s log of each label.
- `evidence/`: notes (UTC, one line per step); every `count20.py`, `nm.py` and `st.py` output, whole, with its exit; each send's UTC time and request check; each background block's output and exit; the empty-answer checks; the three Gateway images' list; the per-label wall clock; the book check; `tally.exe`'s file data. Process, port, folder and backup listings and the restore plan stay on the lab PC.
- `scripts/`: the sitting's helpers. `run.py`, `count20.py`, `nm.py`, `st.py` and `rf.py` are the sheet's files, byte for byte, and are not repeated.
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

### p1 empty-answer check
```
p1 L-lo-pl-20260801.xml: run.py stopped (log-p1.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T22:11:13.428Z GET /status after p1: curl 0 "status=200, bytes=51"
```

### p1 nm.py
```
== a-p1-L-lo-pl-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p1-L-lo-pl-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p2 empty-answer check
```
p2 L-lo-cm-20260801.xml: run.py stopped (log-p2.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T22:11:24.391Z GET /status after p2: curl 0 "status=200, bytes=51"
```

### p2 nm.py
```
== a-p2-L-lo-cm-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p2-L-lo-cm-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p3 empty-answer check
```
p3 L-lo-us-20260801.xml: run.py stopped (log-p3.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T22:11:29.130Z GET /status after p3: curl 0 "status=200, bytes=51"
```

### p3 nm.py
```
== a-p3-L-lo-us-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p3-L-lo-us-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p4 empty-answer check
```
p4 L-lo-dq-20260801.xml: run.py stopped (log-p4.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T22:11:31.871Z GET /status after p4: curl 0 "status=200, bytes=51"
```

### p4 nm.py
```
== a-p4-L-lo-dq-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p4-L-lo-dq-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p5 empty-answer check
```
p5 L-lo-ps-20260801.xml: run.py stopped (log-p5.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T22:11:34.688Z GET /status after p5: curl 0 "status=200, bytes=51"
```

### p5 nm.py
```
== a-p5-L-lo-ps-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p5-L-lo-ps-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p6 empty-answer check
```
p6 L-lo-at-20260801.xml: run.py stopped (log-p6.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T22:11:37.554Z GET /status after p6: curl 0 "status=200, bytes=51"
```

### p6 nm.py
```
== a-p6-L-lo-at-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p6-L-lo-at-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p7 empty-answer check
```
p7 L-lo-hs-20260801.xml: run.py stopped (log-p7.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T22:11:40.321Z GET /status after p7: curl 0 "status=200, bytes=51"
```

### p7 nm.py
```
== a-p7-L-lo-hs-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p7-L-lo-hs-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p8 empty-answer check
```
p8 L-lo-cl-20260801.xml: run.py stopped (log-p8.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T22:11:43.215Z GET /status after p8: curl 0 "status=200, bytes=51"
```

### p8 nm.py
```
== a-p8-L-lo-cl-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p8-L-lo-cl-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p9 empty-answer check
```
p9 L-lo-pc-20260801.xml: run.py stopped (log-p9.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T22:11:45.977Z GET /status after p9: curl 0 "status=200, bytes=51"
```

### p9 nm.py
```
== a-p9-L-lo-pc-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p9-L-lo-pc-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p10 empty-answer check
```
p10 L-lo-st-20260801.xml: run.py stopped (log-p10.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T22:11:48.643Z GET /status after p10: curl 0 "status=200, bytes=51"
```

### p10 nm.py
```
== a-p10-L-lo-st-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p10-L-lo-st-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p11 empty-answer check
```
p11 L-lo-ex-20260801.xml: run.py stopped (log-p11.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T22:11:51.276Z GET /status after p11: curl 0 "status=200, bytes=51"
```

### p11 nm.py
```
== a-p11-L-lo-ex-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p11-L-lo-ex-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p12 count20.py --same-row
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

### r1 (pl) nm.py
```
== a-r1a-L-lo-pl-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0261' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r1 nm.py --same
```
  a-r1a-L-lo-pl-20260801.xml: 896 bytes, sha256 112a4160a64e99742b42ba690d595e3456d49290b3380503d163eac3dfcca67f
  a-r1b-L-lo-pl-20260801.xml: 896 bytes, sha256 112a4160a64e99742b42ba690d595e3456d49290b3380503d163eac3dfcca67f
SAME: EQUAL
exit 0
```

### r1 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39182']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39182']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0261' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r1 nm.py --against
```
  BILLREF 'BP/26-27/0261' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Nx Plain': 1
    BILLREF 'BP/26-27/0261'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0261): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r2 (cm) nm.py
```
== a-r2a-L-lo-cm-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0262' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r2 nm.py --same
```
  a-r2a-L-lo-cm-20260801.xml: 896 bytes, sha256 ffc0820068ca9fee973dcaa17b38fe2f6161848faba94c321dfcc622f5725b58
  a-r2b-L-lo-cm-20260801.xml: 896 bytes, sha256 ffc0820068ca9fee973dcaa17b38fe2f6161848faba94c321dfcc622f5725b58
SAME: EQUAL
exit 0
```

### r2 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39182']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39182']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0262' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r2 nm.py --against
```
  BILLREF 'BP/26-27/0262' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Nx Ak, Am': 1
    BILLREF 'BP/26-27/0262'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0262): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r3 (us) nm.py
```
== a-r3a-L-lo-us-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0263' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r3 nm.py --same
```
  a-r3a-L-lo-us-20260801.xml: 896 bytes, sha256 d7abfed5a7e2352a1a423a20392355c981e7f9f7360ce782d3dd140770ffe900
  a-r3b-L-lo-us-20260801.xml: 896 bytes, sha256 d7abfed5a7e2352a1a423a20392355c981e7f9f7360ce782d3dd140770ffe900
SAME: EQUAL
exit 0
```

### r3 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39182']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39182']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0263' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r3 nm.py --against
```
  BILLREF 'BP/26-27/0263' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Nx Bk_Bm': 1
    BILLREF 'BP/26-27/0263'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0263): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r4 (dq) nm.py
```
== a-r4a-L-lo-dq-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0264' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r4 nm.py --same
```
  a-r4a-L-lo-dq-20260801.xml: 896 bytes, sha256 3b313034c1e95ea0f726009ff4b0d3a059a9206159d8bc2de5235deb61d5519d
  a-r4b-L-lo-dq-20260801.xml: 896 bytes, sha256 3b313034c1e95ea0f726009ff4b0d3a059a9206159d8bc2de5235deb61d5519d
SAME: EQUAL
exit 0
```

### r4 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39182']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39182']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0264' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r4 nm.py --against
```
  BILLREF 'BP/26-27/0264' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Nx "Ck" Cm': 1
    BILLREF 'BP/26-27/0264'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0264): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r5 (ps) nm.py
```
== a-r5a-L-lo-ps-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0265' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r5 nm.py --same
```
  a-r5a-L-lo-ps-20260801.xml: 896 bytes, sha256 d908e7593f001f4f8fa6c92ef63a3f7f3226edd8f5c8118731645c7e9d93b5f9
  a-r5b-L-lo-ps-20260801.xml: 896 bytes, sha256 d908e7593f001f4f8fa6c92ef63a3f7f3226edd8f5c8118731645c7e9d93b5f9
SAME: EQUAL
exit 0
```

### r5 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39182']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39182']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0265' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r5 nm.py --against
```
  BILLREF 'BP/26-27/0265' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Nx Dk + Dm': 1
    BILLREF 'BP/26-27/0265'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0265): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r6 (at) nm.py
```
== a-r6a-L-lo-at-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0266' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r6 nm.py --same
```
  a-r6a-L-lo-at-20260801.xml: 896 bytes, sha256 22ad6aa26c058043816ded8324ae45dfe7a7cf8aa93af9ad698d3777e23564a7
  a-r6b-L-lo-at-20260801.xml: 896 bytes, sha256 22ad6aa26c058043816ded8324ae45dfe7a7cf8aa93af9ad698d3777e23564a7
SAME: EQUAL
exit 0
```

### r6 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39182']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39182']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0266' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r6 nm.py --against
```
  BILLREF 'BP/26-27/0266' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Nx Ek @ Em': 1
    BILLREF 'BP/26-27/0266'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0266): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r7 (hs) nm.py
```
== a-r7a-L-lo-hs-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0267' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r7 nm.py --same
```
  a-r7a-L-lo-hs-20260801.xml: 896 bytes, sha256 01707a6b3dbc5bf469c72408552fead2bd353d82b004c1a3d73e7e55c09ff3df
  a-r7b-L-lo-hs-20260801.xml: 896 bytes, sha256 01707a6b3dbc5bf469c72408552fead2bd353d82b004c1a3d73e7e55c09ff3df
SAME: EQUAL
exit 0
```

### r7 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39182']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39182']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0267' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r7 nm.py --against
```
  BILLREF 'BP/26-27/0267' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Nx Fk #12': 1
    BILLREF 'BP/26-27/0267'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0267): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r8 (cl) nm.py
```
== a-r8a-L-lo-cl-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0268' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r8 nm.py --same
```
  a-r8a-L-lo-cl-20260801.xml: 896 bytes, sha256 cd3946a9310c5f5b02988bc6c678601a321902674c769e2300ffc1b1d689e73d
  a-r8b-L-lo-cl-20260801.xml: 896 bytes, sha256 cd3946a9310c5f5b02988bc6c678601a321902674c769e2300ffc1b1d689e73d
SAME: EQUAL
exit 0
```

### r8 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39182']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39182']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0268' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r8 nm.py --against
```
  BILLREF 'BP/26-27/0268' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Nx Gk: Gm': 1
    BILLREF 'BP/26-27/0268'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0268): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r9 (pc) nm.py
```
== a-r9a-L-lo-pc-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0269' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r9 nm.py --same
```
  a-r9a-L-lo-pc-20260801.xml: 896 bytes, sha256 f3973ccc1acb2708af11ed361d494d760eb0dd07059b5586bc9928f2da4bf42a
  a-r9b-L-lo-pc-20260801.xml: 896 bytes, sha256 f3973ccc1acb2708af11ed361d494d760eb0dd07059b5586bc9928f2da4bf42a
SAME: EQUAL
exit 0
```

### r9 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39182']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39182']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0269' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r9 nm.py --against
```
  BILLREF 'BP/26-27/0269' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Nx Hk 15% Hm': 1
    BILLREF 'BP/26-27/0269'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0269): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r10 (st) nm.py
```
== a-r10a-L-lo-st-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0270' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r10 nm.py --same
```
  a-r10a-L-lo-st-20260801.xml: 896 bytes, sha256 beaea943a92fb4c5544137fa5edac1b836def53d56199e40ad076d092751bbf3
  a-r10b-L-lo-st-20260801.xml: 896 bytes, sha256 beaea943a92fb4c5544137fa5edac1b836def53d56199e40ad076d092751bbf3
SAME: EQUAL
exit 0
```

### r10 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39182']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39182']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0270' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r10 nm.py --against
```
  BILLREF 'BP/26-27/0270' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Nx Jk * Jm': 1
    BILLREF 'BP/26-27/0270'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0270): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r11 (ex) nm.py
```
== a-r11a-L-lo-ex-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0271' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r11 nm.py --same
```
  a-r11a-L-lo-ex-20260801.xml: 896 bytes, sha256 388027a631c89db43765e121c70e8c4b5c26ce3de1d82d00ff5e71772c49c34d
  a-r11b-L-lo-ex-20260801.xml: 896 bytes, sha256 388027a631c89db43765e121c70e8c4b5c26ce3de1d82d00ff5e71772c49c34d
SAME: EQUAL
exit 0
```

### r11 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39182']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39182']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0271' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r11 nm.py --against
```
  BILLREF 'BP/26-27/0271' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Nx Kk!': 1
    BILLREF 'BP/26-27/0271'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0271): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### m1 count20.py --same-row
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
M-marks.xml 3394 B byte-equal
```

### s9 count20.py --same-row against s1
```
== a-s9-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
exit 0
```
