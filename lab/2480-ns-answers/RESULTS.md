# Row 2480-NS: all eleven names answered with no bytes before their ledgers existed; each created as one new ledger with its own bill; each then read by the report as its own ledger, told and equal to the other two reads

Run on 10 Nov 2026, 20:03:40Z to 20:24:36Z (01:33 to 01:54 IST, 11 Nov), on TallyPrime single-user edition: the Gateway on 9001, `tally.exe` file version 1.1.7.2 (`evidence/tally-exe.txt`; the Gateway shows no release number). The sheet was `lab/2480-ns` (6e44a3dc), with all 74 files equal to its `SHA256.txt` (sizes too), and `LAB-COMMON.md` byte-equal to the one row 2440-ED ran under. One book, `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic, in the synthetic-uuid registry), loaded alone, in a sitting of its own with a Tally start of its own. No Bridge build ran and no Bridge tool was called. Each request file's sha256 and size were checked against the sheet's `SHA256.txt` just before its send (`evidence/sends-utc.txt`).

Every value below is M, measured by this run, in one run. Every read was answered, each with curl exit 0 and HTTP 200; no dialog, message or prompt showed but the two of C6 (none at the backup, the loads or the shuts), and every capture after a send was the Gateway, byte-equal to the first.

## Result

**Step 3, before any ledger existed (p1 to p11).** Each of the eleven `L-lo-<id>-20260801.xml` requests was answered HTTP 200 with `CONTENT-LENGTH: 0` and 0 bytes: `nm.py` `ANSWER: NO BYTES` for each, the control included. `run.py` reported each as "empty answer"; each was recorded under the run lead's empty-answer exception (Deviations), each followed by one GET /status that answered 200 (51 B). **So no name was answered with another ledger's bills before its own ledger existed.** p12's marks were equal to s1's (ALTVCHID 68, ALTMSTID 245): the eleven reads changed nothing.

**Step 4, the set-up.** All 22 imports, each sent once, in table order, each after a marks read that passed `count20.py <the before-write option>`: every answer `CREATED` 1 and `ALTERED`, `DELETED`, `COMBINED`, `IGNORED`, `ERRORS`, `CANCELLED` and `EXCEPTIONS` each 0, with no LINEERROR, so `st.py --import` gave `IMPORT: CREATED ONE` (exit 0) on each. The invoices' `LASTVCHID` ran from 45 to 55 in table order. **The five names that NM's row saw answered as an alteration (dot, ampersand, slash, the two brackets) were each created as a new ledger here.** The marks rose by one master per ledger and one voucher per invoice: 68/245 before the first write, 79/256 after the last (s6m).

**Step 5, the book-wide reads.** The snapshot, sent twice (s6sa, s6sb): 39,154 B each, byte-equal, 29 LEDGER rows (18 at s2), a row for each of the eleven, each `Sundry Debtors`, bill-wise Yes, opening 0.00, closing -1051.00. B-bills, sent twice (s6ba, s6bb): 5,098 B each, byte-equal, 11 BILLFIXED groups, one for each name, `BP/26-27/0221` to `BP/26-27/0231` in table order, each dated 1-Aug-26 and -1051.00, no group of any other party. `st.py --names`: `NAMES: EQUAL`, each ledger's name list its own name alone. **The intended state, as the sheet defines it.**

**Step 6, the report reads (r1a to r11b).** Each id has a ledger and a bill as step 6 defines it, so all eleven were read, each twice:

| n | id | name | r<n>a, r<n>b | rows | the bill | `--same` | `--outcome` | `--against` |
|---|---|---|---|---|---|---|---|---|
| 1 | `pl` | `BRIDGE Ns Plain` | 896 B each | 1 (0 On Account) | `BP/26-27/0221`, -1051.00 | EQUAL | told | EQUAL |
| 2 | `hy` | `BRIDGE Ns Hk-Hm` | 896 B each | 1 (0 On Account) | `BP/26-27/0222`, -1051.00 | EQUAL | told | EQUAL |
| 3 | `dt` | `BRIDGE Ns Dk.Dm` | 896 B each | 1 (0 On Account) | `BP/26-27/0223`, -1051.00 | EQUAL | told | EQUAL |
| 4 | `am` | `BRIDGE Ns Ak&Am` | 896 B each | 1 (0 On Account) | `BP/26-27/0224`, -1051.00 | EQUAL | told | EQUAL |
| 5 | `sl` | `BRIDGE Ns Sk/Sm` | 896 B each | 1 (0 On Account) | `BP/26-27/0225`, -1051.00 | EQUAL | told | EQUAL |
| 6 | `lp` | `BRIDGE Ns Lk(Lm` | 896 B each | 1 (0 On Account) | `BP/26-27/0226`, -1051.00 | EQUAL | told | EQUAL |
| 7 | `rp` | `BRIDGE Ns Rk)Rm` | 896 B each | 1 (0 On Account) | `BP/26-27/0227`, -1051.00 | EQUAL | told | EQUAL |
| 8 | `ap` | `BRIDGE Ns Pk'Pm` | 896 B each | 1 (0 On Account) | `BP/26-27/0228`, -1051.00 | EQUAL | told | EQUAL |
| 9 | `op` | `BRIDGE Ns Ok and Om` | 896 B each | 1 (0 On Account) | `BP/26-27/0229`, -1051.00 | EQUAL | told | EQUAL |
| 10 | `dd` | `BRIDGE Ns 2023-24` | 896 B each | 1 (0 On Account) | `BP/26-27/0230`, -1051.00 | EQUAL | told | EQUAL |
| 11 | `sc` | `BRIDGE Ns उदाहरण` | 896 B each | 1 (0 On Account) | `BP/26-27/0231`, -1051.00 | EQUAL | told | EQUAL |

Each bill is dated 1-Aug-26, due 1-Aug-26, overdue 0. `--outcome` told: the sum of the bills (-1051.00) equals the snapshot's closing for that ledger. `--against` EQUAL: the report's rows equal that ledger's B-bills group. **The control is told with `AGAINST: EQUAL`, so each other name is judged against it: by the sheet's Decides, for each of the ten marks, on this book and release, Tally read the name in the `LedgerName` static variable as that ledger and as no other** (NO BYTES at step 3; told, `AGAINST: EQUAL`, the one bill its own, the two sends equal at step 6). It says nothing about the formula read (`CHILDOF`).

**Step 7.** m1's marks were equal to s6m's (79/256): steps 5 and 6 only read.

**Step 8, the way back.** BK was restored by C6, with only the two expected prompts. Then s9 sent every file of s2: every answer byte-equal to its s2 answer (`evidence/s9-against-s2.txt`), and the marks equal to s1's (68/245; `count20.py --same-row`). So the book holds again what it held at step 1.

**Timings.** Every read sent without a window capture after it returned within 1 s of its send line, wall clock, python's start included; s2's fourteen reads took 1.826 s together and s9's 1.948 s. The labels sent with a capture took 2.679 to 3.607 s, the capture included (`evidence/durations.txt`).

**What this does not say.** Whether the build's own parse accepts these answers; the formula read (`CHILDOF`); any other book, release or mark; names that share a stem (row 2480-NM's question); the composed path through a built executable.

## Deviations from the sheet

- **`python` for `python3`.** This PC has no `python3` command. Every script ran as `python -I` (Python 3.10), unchanged, from the row folder, through the helpers in `scripts/`.
- **The empty-answer exception.** `run.py` reports each empty L-lo- answer as a failed request. The run lead's word for this row (in the notes, 20:03Z) made an empty L-lo- answer with HTTP 200 and `CONTENT-LENGTH: 0` at p1 to p11 or at any r<n>a or r<n>b not a stop, each followed by GET /status once. The helper `scripts/lr.sh` tested exactly that (headers status 200, `CONTENT-LENGTH: 0`, a 0-byte file) before the next request; each p answer met it (`evidence/empty-p*.txt`). No r answer was empty.
- **The start-up company** loaded by itself when Tally started. It was shut (K: Company, Shut, Enter on its one row, no prompt) before the book was loaded, on the run lead's standing word.
- **The three Bridge processes** that ran before the row (two `bridge_mcp.exe` and `BridgeCommunication.exe`) were left running, as the sheet says; no process is ended on this PC. None was called by the row.
- **s9's order.** C6 says the reads after a restore begin with MARKS; the sheet says s2's order, which puts M-marks last. s9 sent M-marks first, then the other thirteen files in s2's order. Each answer is compared whole with its s2 answer, so the order changes no comparison.
- **The wait between labels.** Step 3's p2 to p11 and step 6's reads were each sent by a loop that went on only when the previous label's checks passed and its window capture was byte-equal to p1's; step 4 was sent by hand, one label a command.

## Steps
- Step 0 (20:03:40Z): the sheet's files checked; the book's data folder found once by name and GUID; the excluded GUID absent (`evidence/bookcheck.txt`). The backup folder made new and empty. No `tally.exe` ran.
- Tally started from its desktop shortcut (20:03:50Z). The start-up company was shut (Deviations). `BRIDGE`, space, `PILOT`, space, `LAB` typed in Select Company's filter; Enter only after a capture showed the one row left (`s1-select`).
- Gateway: the book alone; period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026; last entry 15-Sep-26; no Educational mode. Step 0 C2 (20:06:24Z): one `tally.exe`, which alone owns 9001; GET /status status=200, bytes=51. s1: the book's GUID, ALTVCHID 68, ALTMSTID 245, as the sheet's state.
- BK by C5 (capture before Accept 20:08:16Z): Data, Backup & Restore, Backup; Backspace to the path, Specify Path, `C:\lab\2480-ns\backup-1` (Tally showed `C:\lab\2480-Ns\backup-1`); the book the one company; End of List; Accept y; back at the Gateway with no message. One file of 90,051 B, at 01:38 IST, 11 Nov (`evidence/bk.txt`).
- Step 2 (s2): each N read 0 rows; B-bills no group; the snapshot 18 LEDGER rows and no row for any of the eleven; `st.py <the before option>` `FOLD: CLEAR`; `st.py --names` `NAMES: EQUAL`; the marks 68/245 (C5's marks read after the backup).
- Step 3 (20:10:00Z to 20:11:15Z), step 4 (20:11:49Z to 20:17:52Z), step 5, step 6 and step 7 as in Result, each in the notes.
- Step 8 C6 (prep 20:20:55Z: the BK re-hashed equal, its folder holding that file alone, `restore-plan.txt` written, the data folder listed): the book shut by K: Company, Shut, Enter, no prompt; Data, Backup & Restore, Restore; Restore Data Path Tally's data folder; Specify Path, the backup-1 folder; the list showed the book, its number and 11-Nov-26 (1:38 am), as the plan; the book selected alone; Accept ? y (20:22:23Z); the folder-exists question with the book's number (20:22:39Z), r Replace; back at Select Company with no other prompt; the data folder listed again. The book loaded alone by the name filter; Gateway captured, date and period unchanged; then s9.
- Step 9, C10 (20:24:11Z): the book shut by K: Company, Shut, Enter on its one row, no prompt; Select Company with none loaded; one `tally.exe` owning 9001; GET /status status=200, bytes=51. No request after the shut. Tally then quit by its own Quit, no prompt; at 20:24:36Z no `tally.exe` ran and nothing listened on 9001.

## Files
- `answers/`: every answer and its headers file, whole, byte for byte (`kept-local.txt` lists any that stay on the lab PC).
- `logs/`: `run.py`'s log of each label.
- `evidence/`: notes (UTC, one line per step); every `count20.py`, `nm.py` and `st.py` output, whole, with its exit; each send's UTC time and request check; the empty-answer checks; the per-label wall clock; the book check; `tally.exe`'s file data. Process, port, folder and backup listings and the restore plan stay on the lab PC.
- `scripts/`: the sitting's helpers. `run.py`, `count20.py`, `nm.py` and `st.py` ran as the sheet's files, byte for byte, and are not repeated.
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
== a-s2-N-0221.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0222.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0223.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0224.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0225.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0226.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0227.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0228.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0229.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0230.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0231.xml: 3022 bytes
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
  pl 'BRIDGE Ns Plain': 0 groups
  hy 'BRIDGE Ns Hk-Hm': 0 groups
  dt 'BRIDGE Ns Dk.Dm': 0 groups
  am 'BRIDGE Ns Ak&Am': 0 groups
  sl 'BRIDGE Ns Sk/Sm': 0 groups
  lp 'BRIDGE Ns Lk(Lm': 0 groups
  rp 'BRIDGE Ns Rk)Rm': 0 groups
  ap "BRIDGE Ns Pk'Pm": 0 groups
  op 'BRIDGE Ns Ok and Om': 0 groups
  dd 'BRIDGE Ns 2023-24': 0 groups
  sc 'BRIDGE Ns उदाहरण': 0 groups
  groups of any other party: 0
== a-s2-S-snap-20260401-20260801.xml
a-s2-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 18
  pl 'BRIDGE Ns Plain': no row
  hy 'BRIDGE Ns Hk-Hm': no row
  dt 'BRIDGE Ns Dk.Dm': no row
  am 'BRIDGE Ns Ak&Am': no row
  sl 'BRIDGE Ns Sk/Sm': no row
  lp 'BRIDGE Ns Lk(Lm': no row
  rp 'BRIDGE Ns Rk)Rm': no row
  ap "BRIDGE Ns Pk'Pm": no row
  op 'BRIDGE Ns Ok and Om': no row
  dd 'BRIDGE Ns 2023-24': no row
  sc 'BRIDGE Ns उदाहरण': no row
exit 0
```

### s2 st.py <the before option>
```
a-s2-S-snap-20260401-20260801.xml: LEDGER rows 18
FOLD: CLEAR
```

### s2 st.py --names
```
a-s2-S-snap-20260401-20260801.xml: LEDGER rows 18
  pl 'BRIDGE Ns Plain': no row
  hy 'BRIDGE Ns Hk-Hm': no row
  dt 'BRIDGE Ns Dk.Dm': no row
  am 'BRIDGE Ns Ak&Am': no row
  sl 'BRIDGE Ns Sk/Sm': no row
  lp 'BRIDGE Ns Lk(Lm': no row
  rp 'BRIDGE Ns Rk)Rm': no row
  ap "BRIDGE Ns Pk'Pm": no row
  op 'BRIDGE Ns Ok and Om': no row
  dd 'BRIDGE Ns 2023-24': no row
  sc 'BRIDGE Ns उदाहरण': no row
NAMES: EQUAL
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
2026-11-10T20:10:03.133Z GET /status after p1: curl 0 "status=200, bytes=51"
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
p2 L-lo-hy-20260801.xml: run.py stopped (log-p2.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T20:10:21.173Z GET /status after p2: curl 0 "status=200, bytes=51"
```

### p2 nm.py
```
== a-p2-L-lo-hy-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p2-L-lo-hy-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p3 empty-answer check
```
p3 L-lo-dt-20260801.xml: run.py stopped (log-p3.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T20:10:26.764Z GET /status after p3: curl 0 "status=200, bytes=51"
```

### p3 nm.py
```
== a-p3-L-lo-dt-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p3-L-lo-dt-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p4 empty-answer check
```
p4 L-lo-am-20260801.xml: run.py stopped (log-p4.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T20:10:32.671Z GET /status after p4: curl 0 "status=200, bytes=51"
```

### p4 nm.py
```
== a-p4-L-lo-am-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p4-L-lo-am-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p5 empty-answer check
```
p5 L-lo-sl-20260801.xml: run.py stopped (log-p5.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T20:10:39.207Z GET /status after p5: curl 0 "status=200, bytes=51"
```

### p5 nm.py
```
== a-p5-L-lo-sl-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p5-L-lo-sl-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p6 empty-answer check
```
p6 L-lo-lp-20260801.xml: run.py stopped (log-p6.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T20:10:45.251Z GET /status after p6: curl 0 "status=200, bytes=51"
```

### p6 nm.py
```
== a-p6-L-lo-lp-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p6-L-lo-lp-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p7 empty-answer check
```
p7 L-lo-rp-20260801.xml: run.py stopped (log-p7.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T20:10:51.663Z GET /status after p7: curl 0 "status=200, bytes=51"
```

### p7 nm.py
```
== a-p7-L-lo-rp-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p7-L-lo-rp-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p8 empty-answer check
```
p8 L-lo-ap-20260801.xml: run.py stopped (log-p8.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T20:10:58.222Z GET /status after p8: curl 0 "status=200, bytes=51"
```

### p8 nm.py
```
== a-p8-L-lo-ap-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p8-L-lo-ap-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p9 empty-answer check
```
p9 L-lo-op-20260801.xml: run.py stopped (log-p9.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T20:11:03.880Z GET /status after p9: curl 0 "status=200, bytes=51"
```

### p9 nm.py
```
== a-p9-L-lo-op-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p9-L-lo-op-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p10 empty-answer check
```
p10 L-lo-dd-20260801.xml: run.py stopped (log-p10.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T20:11:09.302Z GET /status after p10: curl 0 "status=200, bytes=51"
```

### p10 nm.py
```
== a-p10-L-lo-dd-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p10-L-lo-dd-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p11 empty-answer check
```
p11 L-lo-sc-20260801.xml: run.py stopped (log-p11.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T20:11:14.768Z GET /status after p11: curl 0 "status=200, bytes=51"
```

### p11 nm.py
```
== a-p11-L-lo-sc-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p11-L-lo-sc-20260801.xml: 0 bytes
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

### s4hym count20.py (the before-write option)
```
== a-s4hym-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4hy st.py --import
```
a-s4hy-LM-hy.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5hym count20.py (the before-write option)
```
== a-s5hym-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'247'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5hy st.py --import
```
a-s5hy-I-hy.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 46, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4dtm count20.py (the before-write option)
```
== a-s4dtm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'247'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4dt st.py --import
```
a-s4dt-LM-dt.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5dtm count20.py (the before-write option)
```
== a-s5dtm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5dt st.py --import
```
a-s5dt-I-dt.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 47, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4amm count20.py (the before-write option)
```
== a-s4amm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4am st.py --import
```
a-s4am-LM-am.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5amm count20.py (the before-write option)
```
== a-s5amm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'249'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5am st.py --import
```
a-s5am-I-am.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 48, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4slm count20.py (the before-write option)
```
== a-s4slm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'249'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4sl st.py --import
```
a-s4sl-LM-sl.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5slm count20.py (the before-write option)
```
== a-s5slm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'250'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5sl st.py --import
```
a-s5sl-I-sl.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 49, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4lpm count20.py (the before-write option)
```
== a-s4lpm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'73'", "'250'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4lp st.py --import
```
a-s4lp-LM-lp.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5lpm count20.py (the before-write option)
```
== a-s5lpm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'73'", "'251'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5lp st.py --import
```
a-s5lp-I-lp.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 50, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4rpm count20.py (the before-write option)
```
== a-s4rpm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'251'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4rp st.py --import
```
a-s4rp-LM-rp.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5rpm count20.py (the before-write option)
```
== a-s5rpm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5rp st.py --import
```
a-s5rp-I-rp.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 51, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4apm count20.py (the before-write option)
```
== a-s4apm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'75'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4ap st.py --import
```
a-s4ap-LM-ap.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5apm count20.py (the before-write option)
```
== a-s5apm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'75'", "'253'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5ap st.py --import
```
a-s5ap-I-ap.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 52, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4opm count20.py (the before-write option)
```
== a-s4opm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'76'", "'253'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4op st.py --import
```
a-s4op-LM-op.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5opm count20.py (the before-write option)
```
== a-s5opm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'76'", "'254'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5op st.py --import
```
a-s5op-I-op.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 53, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4ddm count20.py (the before-write option)
```
== a-s4ddm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'77'", "'254'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4dd st.py --import
```
a-s4dd-LM-dd.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5ddm count20.py (the before-write option)
```
== a-s5ddm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'77'", "'255'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5dd st.py --import
```
a-s5dd-I-dd.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 54, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4scm count20.py (the before-write option)
```
== a-s4scm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'78'", "'255'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4sc st.py --import
```
a-s4sc-LM-sc.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5scm count20.py (the before-write option)
```
== a-s5scm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'78'", "'256'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5sc st.py --import
```
a-s5sc-I-sc.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 55, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
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
  pl 'BRIDGE Ns Plain': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  hy 'BRIDGE Ns Hk-Hm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  dt 'BRIDGE Ns Dk.Dm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  am 'BRIDGE Ns Ak&Am': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  sl 'BRIDGE Ns Sk/Sm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  lp 'BRIDGE Ns Lk(Lm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  rp 'BRIDGE Ns Rk)Rm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ap "BRIDGE Ns Pk'Pm": PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  op 'BRIDGE Ns Ok and Om': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  dd 'BRIDGE Ns 2023-24': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  sc 'BRIDGE Ns उदाहरण': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
exit 0
```

### s6ba nm.py
```
== a-s6ba-B-bills.xml
  BILLFIXED groups: 11
  pl 'BRIDGE Ns Plain': 1 groups
    BILLREF 'BP/26-27/0221'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  hy 'BRIDGE Ns Hk-Hm': 1 groups
    BILLREF 'BP/26-27/0222'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  dt 'BRIDGE Ns Dk.Dm': 1 groups
    BILLREF 'BP/26-27/0223'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  am 'BRIDGE Ns Ak&Am': 1 groups
    BILLREF 'BP/26-27/0224'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  sl 'BRIDGE Ns Sk/Sm': 1 groups
    BILLREF 'BP/26-27/0225'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  lp 'BRIDGE Ns Lk(Lm': 1 groups
    BILLREF 'BP/26-27/0226'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  rp 'BRIDGE Ns Rk)Rm': 1 groups
    BILLREF 'BP/26-27/0227'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ap "BRIDGE Ns Pk'Pm": 1 groups
    BILLREF 'BP/26-27/0228'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  op 'BRIDGE Ns Ok and Om': 1 groups
    BILLREF 'BP/26-27/0229'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  dd 'BRIDGE Ns 2023-24': 1 groups
    BILLREF 'BP/26-27/0230'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  sc 'BRIDGE Ns उदाहरण': 1 groups
    BILLREF 'BP/26-27/0231'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  groups of any other party: 0
exit 0
```

### s6sa/s6sb nm.py --same
```
  a-s6sa-S-snap-20260401-20260801.xml: 39154 bytes, sha256 6338e8c0eca34346065898d02fce949b988917aad5a4a3bd6283dfe911d05462
  a-s6sb-S-snap-20260401-20260801.xml: 39154 bytes, sha256 6338e8c0eca34346065898d02fce949b988917aad5a4a3bd6283dfe911d05462
SAME: EQUAL
exit 0
```

### s6ba/s6bb nm.py --same
```
  a-s6ba-B-bills.xml: 5098 bytes, sha256 b3103d4cc08d62071c2d267806362c9fea2d6ebf3d6352c41c7a52814d7107ef
  a-s6bb-B-bills.xml: 5098 bytes, sha256 b3103d4cc08d62071c2d267806362c9fea2d6ebf3d6352c41c7a52814d7107ef
SAME: EQUAL
exit 0
```

### s6 st.py --names
```
a-s6sa-S-snap-20260401-20260801.xml: LEDGER rows 29
  pl 'BRIDGE Ns Plain': names ['BRIDGE Ns Plain']
  hy 'BRIDGE Ns Hk-Hm': names ['BRIDGE Ns Hk-Hm']
  dt 'BRIDGE Ns Dk.Dm': names ['BRIDGE Ns Dk.Dm']
  am 'BRIDGE Ns Ak&Am': names ['BRIDGE Ns Ak&Am']
  sl 'BRIDGE Ns Sk/Sm': names ['BRIDGE Ns Sk/Sm']
  lp 'BRIDGE Ns Lk(Lm': names ['BRIDGE Ns Lk(Lm']
  rp 'BRIDGE Ns Rk)Rm': names ['BRIDGE Ns Rk)Rm']
  ap "BRIDGE Ns Pk'Pm": names ["BRIDGE Ns Pk'Pm"]
  op 'BRIDGE Ns Ok and Om': names ['BRIDGE Ns Ok and Om']
  dd 'BRIDGE Ns 2023-24': names ['BRIDGE Ns 2023-24']
  sc 'BRIDGE Ns उदाहरण': names ['BRIDGE Ns उदाहरण']
NAMES: EQUAL
exit 0
```

### r1 (pl) nm.py
```
== a-r1a-L-lo-pl-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0221' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r1 nm.py --same
```
  a-r1a-L-lo-pl-20260801.xml: 896 bytes, sha256 45f34d9819048484aa555bbfd17b4c74cdcd1f10a51ee24f9e3a344747fbb877
  a-r1b-L-lo-pl-20260801.xml: 896 bytes, sha256 45f34d9819048484aa555bbfd17b4c74cdcd1f10a51ee24f9e3a344747fbb877
SAME: EQUAL
exit 0
```

### r1 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39154']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39154']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0221' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r1 nm.py --against
```
  BILLREF 'BP/26-27/0221' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Ns Plain': 1
    BILLREF 'BP/26-27/0221'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0221): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r2 (hy) nm.py
```
== a-r2a-L-lo-hy-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0222' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r2 nm.py --same
```
  a-r2a-L-lo-hy-20260801.xml: 896 bytes, sha256 9106fcd53bad280111db32938332c7da1f1a8831ac5371cc4e78523bf29ef1b4
  a-r2b-L-lo-hy-20260801.xml: 896 bytes, sha256 9106fcd53bad280111db32938332c7da1f1a8831ac5371cc4e78523bf29ef1b4
SAME: EQUAL
exit 0
```

### r2 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39154']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39154']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0222' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r2 nm.py --against
```
  BILLREF 'BP/26-27/0222' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Ns Hk-Hm': 1
    BILLREF 'BP/26-27/0222'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0222): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r3 (dt) nm.py
```
== a-r3a-L-lo-dt-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0223' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r3 nm.py --same
```
  a-r3a-L-lo-dt-20260801.xml: 896 bytes, sha256 81c33e352aaa4a406041e112373a6b91a3d52a25da0a811939adb07dae63da34
  a-r3b-L-lo-dt-20260801.xml: 896 bytes, sha256 81c33e352aaa4a406041e112373a6b91a3d52a25da0a811939adb07dae63da34
SAME: EQUAL
exit 0
```

### r3 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39154']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39154']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0223' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r3 nm.py --against
```
  BILLREF 'BP/26-27/0223' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Ns Dk.Dm': 1
    BILLREF 'BP/26-27/0223'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0223): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r4 (am) nm.py
```
== a-r4a-L-lo-am-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0224' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r4 nm.py --same
```
  a-r4a-L-lo-am-20260801.xml: 896 bytes, sha256 7105759d757e21ba5281ea1ef859545698446ce4f77b047034021da348fac98b
  a-r4b-L-lo-am-20260801.xml: 896 bytes, sha256 7105759d757e21ba5281ea1ef859545698446ce4f77b047034021da348fac98b
SAME: EQUAL
exit 0
```

### r4 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39154']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39154']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0224' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r4 nm.py --against
```
  BILLREF 'BP/26-27/0224' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Ns Ak&Am': 1
    BILLREF 'BP/26-27/0224'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0224): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r5 (sl) nm.py
```
== a-r5a-L-lo-sl-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0225' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r5 nm.py --same
```
  a-r5a-L-lo-sl-20260801.xml: 896 bytes, sha256 59ef6d45929487f74b4c85e653bbfc9526133927e026e593a6c09be27f17e2ec
  a-r5b-L-lo-sl-20260801.xml: 896 bytes, sha256 59ef6d45929487f74b4c85e653bbfc9526133927e026e593a6c09be27f17e2ec
SAME: EQUAL
exit 0
```

### r5 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39154']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39154']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0225' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r5 nm.py --against
```
  BILLREF 'BP/26-27/0225' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Ns Sk/Sm': 1
    BILLREF 'BP/26-27/0225'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0225): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r6 (lp) nm.py
```
== a-r6a-L-lo-lp-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0226' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r6 nm.py --same
```
  a-r6a-L-lo-lp-20260801.xml: 896 bytes, sha256 dfca8eb42cf152e96ba8c6c75de5c385f53cb9461a1bcf61235349270fa971b4
  a-r6b-L-lo-lp-20260801.xml: 896 bytes, sha256 dfca8eb42cf152e96ba8c6c75de5c385f53cb9461a1bcf61235349270fa971b4
SAME: EQUAL
exit 0
```

### r6 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39154']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39154']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0226' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r6 nm.py --against
```
  BILLREF 'BP/26-27/0226' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Ns Lk(Lm': 1
    BILLREF 'BP/26-27/0226'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0226): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r7 (rp) nm.py
```
== a-r7a-L-lo-rp-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0227' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r7 nm.py --same
```
  a-r7a-L-lo-rp-20260801.xml: 896 bytes, sha256 6b1e1ad206f91b0e9a2dd37e812f79ceeb30b8bdac90ea4f2d33c36d773bfa84
  a-r7b-L-lo-rp-20260801.xml: 896 bytes, sha256 6b1e1ad206f91b0e9a2dd37e812f79ceeb30b8bdac90ea4f2d33c36d773bfa84
SAME: EQUAL
exit 0
```

### r7 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39154']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39154']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0227' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r7 nm.py --against
```
  BILLREF 'BP/26-27/0227' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Ns Rk)Rm': 1
    BILLREF 'BP/26-27/0227'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0227): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r8 (ap) nm.py
```
== a-r8a-L-lo-ap-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0228' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r8 nm.py --same
```
  a-r8a-L-lo-ap-20260801.xml: 896 bytes, sha256 099644837f6f566193215369d20a2ef0eb40beabe1f0e7b94d84ddc8f40e3a3d
  a-r8b-L-lo-ap-20260801.xml: 896 bytes, sha256 099644837f6f566193215369d20a2ef0eb40beabe1f0e7b94d84ddc8f40e3a3d
SAME: EQUAL
exit 0
```

### r8 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39154']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39154']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0228' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r8 nm.py --against
```
  BILLREF 'BP/26-27/0228' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named "BRIDGE Ns Pk'Pm": 1
    BILLREF 'BP/26-27/0228'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0228): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r9 (op) nm.py
```
== a-r9a-L-lo-op-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0229' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r9 nm.py --same
```
  a-r9a-L-lo-op-20260801.xml: 896 bytes, sha256 15fbc04f4a9f48a0467dd405d869fe59cf16b4fc4fb6106fe61f3d7f10f1787d
  a-r9b-L-lo-op-20260801.xml: 896 bytes, sha256 15fbc04f4a9f48a0467dd405d869fe59cf16b4fc4fb6106fe61f3d7f10f1787d
SAME: EQUAL
exit 0
```

### r9 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39154']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39154']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0229' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r9 nm.py --against
```
  BILLREF 'BP/26-27/0229' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Ns Ok and Om': 1
    BILLREF 'BP/26-27/0229'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0229): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r10 (dd) nm.py
```
== a-r10a-L-lo-dd-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0230' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r10 nm.py --same
```
  a-r10a-L-lo-dd-20260801.xml: 896 bytes, sha256 2b4b8b82e2d97c5e9723498a52cf88ea3ded2c4a2466a796b348c37284dc3c95
  a-r10b-L-lo-dd-20260801.xml: 896 bytes, sha256 2b4b8b82e2d97c5e9723498a52cf88ea3ded2c4a2466a796b348c37284dc3c95
SAME: EQUAL
exit 0
```

### r10 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39154']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39154']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0230' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r10 nm.py --against
```
  BILLREF 'BP/26-27/0230' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Ns 2023-24': 1
    BILLREF 'BP/26-27/0230'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0230): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r11 (sc) nm.py
```
== a-r11a-L-lo-sc-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0231' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r11 nm.py --same
```
  a-r11a-L-lo-sc-20260801.xml: 896 bytes, sha256 b9ca8c484b1fcb6ab4b1de9f3acc527a524a2c90b39bdf8c0aaf135b54955465
  a-r11b-L-lo-sc-20260801.xml: 896 bytes, sha256 b9ca8c484b1fcb6ab4b1de9f3acc527a524a2c90b39bdf8c0aaf135b54955465
SAME: EQUAL
exit 0
```

### r11 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39154']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39154']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0231' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r11 nm.py --against
```
  BILLREF 'BP/26-27/0231' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Ns उदाहरण': 1
    BILLREF 'BP/26-27/0231'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0231): yes; sum of BILLCL -1051.00, equals the closing
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
== a-s9-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s9-N-0221.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0222.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0223.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0224.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0225.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0226.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0227.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0228.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0229.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0230.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0231.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
exit 0
```

### s9 (nm.py)
```
== a-s9-B-bills.xml
  BILLFIXED groups: 0
  pl 'BRIDGE Ns Plain': 0 groups
  hy 'BRIDGE Ns Hk-Hm': 0 groups
  dt 'BRIDGE Ns Dk.Dm': 0 groups
  am 'BRIDGE Ns Ak&Am': 0 groups
  sl 'BRIDGE Ns Sk/Sm': 0 groups
  lp 'BRIDGE Ns Lk(Lm': 0 groups
  rp 'BRIDGE Ns Rk)Rm': 0 groups
  ap "BRIDGE Ns Pk'Pm": 0 groups
  op 'BRIDGE Ns Ok and Om': 0 groups
  dd 'BRIDGE Ns 2023-24': 0 groups
  sc 'BRIDGE Ns उदाहरण': 0 groups
  groups of any other party: 0
== a-s9-S-snap-20260401-20260801.xml
a-s9-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 18
  pl 'BRIDGE Ns Plain': no row
  hy 'BRIDGE Ns Hk-Hm': no row
  dt 'BRIDGE Ns Dk.Dm': no row
  am 'BRIDGE Ns Ak&Am': no row
  sl 'BRIDGE Ns Sk/Sm': no row
  lp 'BRIDGE Ns Lk(Lm': no row
  rp 'BRIDGE Ns Rk)Rm': no row
  ap "BRIDGE Ns Pk'Pm": no row
  op 'BRIDGE Ns Ok and Om': no row
  dd 'BRIDGE Ns 2023-24': no row
  sc 'BRIDGE Ns उदाहरण': no row
exit 0
```

### s9 against s2, byte for byte
```
M-marks.xml 3394 B byte-equal
N-0221.xml 3022 B byte-equal
N-0222.xml 3022 B byte-equal
N-0223.xml 3022 B byte-equal
N-0224.xml 3022 B byte-equal
N-0225.xml 3022 B byte-equal
N-0226.xml 3022 B byte-equal
N-0227.xml 3022 B byte-equal
N-0228.xml 3022 B byte-equal
N-0229.xml 3022 B byte-equal
N-0230.xml 3022 B byte-equal
N-0231.xml 3022 B byte-equal
B-bills.xml 46 B byte-equal
S-snap-20260401-20260801.xml 25480 B byte-equal
```

### s9 count20.py --same-row against s1
```
== a-s9-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
exit 0
```
