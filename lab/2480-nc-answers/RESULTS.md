# Row 2480-NC: all eleven mixed names answered with no bytes before their ledgers existed; each created as one new ledger with its own bill; each then read by the report as its own ledger, told and equal to the other two reads

Run on 10 Nov 2026, 21:18:44Z to 21:35:14Z (02:48 to 03:05 IST, 11 Nov), on TallyPrime single-user edition: the Gateway on 9001, `tally.exe` file version 1.1.7.2 (`evidence/tally-exe.txt`; the Gateway shows no release number). The sheet was `lab/2480-nc-sheet` (8eb5012c), with all 75 files equal to its `SHA256.txt` (sizes too); `LAB-COMMON.md`, `run.py`, `count20.py`, `nm.py`, `st.py`, `M-marks.xml`, `B-bills.xml` and the snapshot request byte-equal to row 2480-NS's. One book, `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic, in the synthetic-uuid registry), loaded alone, in a sitting of its own with a Tally start of its own. No Bridge build ran and no Bridge tool was called. Each request file's sha256 and size were checked against the sheet's `SHA256.txt` just before its send (`evidence/sends-utc.txt`).

Every value below is M, measured by this run, in one run. Every read was answered, each with curl exit 0 and HTTP 200; no dialog, message or prompt showed but the two of C6 (none at the backup, the loads or the shuts), and every capture after a send was the Gateway, byte-equal to the first.

## Result

**Step 3, before any ledger existed (p1 to p11).** Each of the eleven `L-lo-<id>-20260801.xml` requests was answered HTTP 200 with `CONTENT-LENGTH: 0` and 0 bytes: `nm.py` `ANSWER: NO BYTES` for each, the control included. `run.py` reported each as "empty answer"; each was recorded under the run lead's empty-answer exception (Deviations), each followed by one GET /status that answered 200 (51 B). **So no name was answered with another ledger's bills before its own ledger existed.** p12's marks were equal to s1's (ALTVCHID 68, ALTMSTID 245): the eleven reads changed nothing.

**Step 4, the set-up.** All 22 imports, each sent once, in table order, each after a marks read that passed `count20.py <the before-write option>`: every answer `CREATED` 1 and `ALTERED`, `DELETED`, `COMBINED`, `IGNORED`, `ERRORS`, `CANCELLED` and `EXCEPTIONS` each 0, with no LINEERROR, so `st.py --import` gave `IMPORT: CREATED ONE` (exit 0) on each. The invoices' `LASTVCHID` ran from 45 to 55 in table order. **Each of the ten names that mix two or more of NS's marks was created as a new ledger.** The marks rose by one master per ledger and one voucher per invoice: 68/245 before the first write, 79/256 after the last (s6m).

**Step 5, the book-wide reads.** The snapshot, sent twice (s6sa, s6sb): 39,634 B each, byte-equal, 29 LEDGER rows (18 at s2), a row for each of the eleven, each `Sundry Debtors`, bill-wise Yes, opening 0.00, closing -1051.00. B-bills, sent twice (s6ba, s6bb): 5,338 B each, byte-equal, 11 BILLFIXED groups, one for each name, `BP/26-27/0241` to `BP/26-27/0251` in table order, each dated 1-Aug-26 and -1051.00, no group of any other party. `st.py --names`: `NAMES: EQUAL`, each ledger's name list its own name alone. **The intended state, as the sheet defines it.**

**Step 6, the report reads (r1a to r11b).** Each id has a ledger and a bill as step 6 defines it, so all eleven were read, each twice:

| n | id | name | r<n>a, r<n>b | rows | the bill | `--same` | `--outcome` | `--against` |
|---|---|---|---|---|---|---|---|---|
| 1 | `pl` | `BRIDGE Nc Plain` | 896 B each | 1 (0 On Account) | `BP/26-27/0241`, -1051.00 | EQUAL | told | EQUAL |
| 2 | `ad` | `BRIDGE Nc Ak & Am Pvt. Ltd.` | 896 B each | 1 (0 On Account) | `BP/26-27/0242`, -1051.00 | EQUAL | told | EQUAL |
| 3 | `sh` | `BRIDGE Nc M/s Bk-Bm` | 896 B each | 1 (0 On Account) | `BP/26-27/0243`, -1051.00 | EQUAL | told | EQUAL |
| 4 | `bd` | `BRIDGE Nc Ck (Cm) Cn.` | 896 B each | 1 (0 On Account) | `BP/26-27/0244`, -1051.00 | EQUAL | told | EQUAL |
| 5 | `qa` | `BRIDGE Nc Dk's & Dm` | 896 B each | 1 (0 On Account) | `BP/26-27/0245`, -1051.00 | EQUAL | told | EQUAL |
| 6 | `wd` | `BRIDGE Nc Ek and Em 2023-24` | 896 B each | 1 (0 On Account) | `BP/26-27/0246`, -1051.00 | EQUAL | told | EQUAL |
| 7 | `ns` | `BRIDGE Nc Fk.Fm/Fn-Fp` | 896 B each | 1 (0 On Account) | `BP/26-27/0247`, -1051.00 | EQUAL | told | EQUAL |
| 8 | `sm` | `BRIDGE Nc उदाहरण & Gk.` | 896 B each | 1 (0 On Account) | `BP/26-27/0248`, -1051.00 | EQUAL | told | EQUAL |
| 9 | `ob` | `BRIDGE Nc (Hk) Hm` | 896 B each | 1 (0 On Account) | `BP/26-27/0249`, -1051.00 | EQUAL | told | EQUAL |
| 10 | `al` | `BRIDGE Nc Jk-Jm. & Jn/Jp (Jq) Jr's and 2023-24` | 896 B each | 1 (0 On Account) | `BP/26-27/0250`, -1051.00 | EQUAL | told | EQUAL |
| 11 | `rs` | `BRIDGE Nc Kk & Sons (India) Pvt. Ltd.` | 896 B each | 1 (0 On Account) | `BP/26-27/0251`, -1051.00 | EQUAL | told | EQUAL |

Each bill is dated 1-Aug-26, due 1-Aug-26, overdue 0. `--outcome` told: the sum of the bills (-1051.00) equals the snapshot's closing for that ledger. `--against` EQUAL: the report's rows equal that ledger's B-bills group. **The control is told with `AGAINST: EQUAL`, so each other name is judged against it: by the sheet's Decides, for each of the ten mixes, on this book and release, Tally read the name in the `LedgerName` static variable as that ledger and as no other** (NO BYTES at step 3; told, `AGAINST: EQUAL`, the one bill its own, the two sends equal at step 6). With row 2480-NS, that is evidence for a type that admits any mix of the measured marks, on one book and release. It says nothing about the formula read (`CHILDOF`).

**Step 7.** m1's marks were equal to s6m's (79/256): steps 5 and 6 only read.

**Step 8, the way back.** BK was restored by C6, with only the two expected prompts. Then s9 sent every file of s2, in s2's order: every answer byte-equal to its s2 answer (`evidence/s9-against-s2.txt`), and the marks equal to s1's (68/245; `count20.py --same-row`). So the book holds again what it held at step 1.

**Timings.** Every read sent without a window capture after it returned within 1 s of its send line, wall clock, python's start included, but s2 and s9: s2's fourteen reads took 2.431 s together and s9's 2.274 s. The labels sent with a capture took 2.718 to 5.568 s, the capture included (`evidence/durations.txt`).

**What this does not say.** Whether the build's own parse accepts these answers; the formula read (`CHILDOF`); any other book, release or mark; which mark or position would cause a failure, since none failed; names that share a stem with another ledger; the composed path through a built executable.

## Deviations from the sheet

- **`python` for `python3`.** This PC has no `python3` command. Every script ran as `python -I` (Python 3.10), unchanged, from the row folder, through the helpers in `scripts/`.
- **The empty-answer exception.** `run.py` reports each empty L-lo- answer as a failed request. The run lead's word for this row (in the queue entry, noted at 21:17Z) made an empty L-lo- answer with HTTP 200 and `CONTENT-LENGTH: 0` not a stop, each followed by GET /status once. The helper `scripts/lr.sh` tested exactly that (headers status 200, `CONTENT-LENGTH: 0`, a 0-byte file) before the next request; each p answer met it (`evidence/empty-p*.txt`). No r answer was empty.
- **The start-up company** loaded by itself when Tally started. It was shut (K: Company, Shut, Enter on its one row, no prompt) before the book was loaded, on the run lead's standing word.
- **The three Bridge processes** that ran before the row (two `bridge_mcp.exe` and `BridgeCommunication.exe`) were left running, as the sheet says; no process is ended on this PC. None was called by the row.
- **Step 4, one id a command.** The sheet asks for one label a command. Each id's four labels (`s4<id>m`, `s4<id>`, `s5<id>m`, `s5<id>`) were sent by one command, `scripts/wid.sh <id>`, which runs `scripts/w.sh` for each in the recipe's order and ends at the first exit other than 0, or at a capture after an import that is not byte-equal to p1's, sending nothing after it. Eleven commands, one per id, in table order; none ended early.
- **A helper call started in error.** Between `wd` and `ns`, a call of `wid.sh pl` was started in error and ended at once. `send.sh` logged no send for it (`evidence/sends-utc.txt` has no line between s5wd's and s4nsm's), and its first label `s4plm` had been used already, which `send.sh` refuses. Nothing was sent to 9001 by it (notes, 21:27Z).
- **The wait between labels.** Step 3's p2 to p11 and step 6's reads were each sent by a loop that went on only when the previous label's helper exited 0 (or 3, bytes came back, at step 6). Each window capture after a send was byte-equal to p1's, checked after the loop.
- **Captures after r<n>b.** `lr.sh` captures the window after each of its sends, so each r<n>b has a capture as well as each r<n>a.

## Steps
- Step 0 (21:18:44Z): the sheet's files checked; the book's data folder found once by name and GUID; the excluded GUID absent (`evidence/bookcheck.txt`). The backup folder made new and empty. No `tally.exe` ran.
- Tally started from its desktop shortcut (21:19:04Z). The start-up company was shut (Deviations). `BRIDGE`, space, `PILOT`, space, `LAB` typed in Select Company's filter; Enter only after a capture showed the one row left (`s1-select`).
- Gateway: the book alone; period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026; last entry 15-Sep-26. Step 0 C2 (21:20:16Z): one `tally.exe`, which alone owns 9001; GET /status status=200, bytes=51. s1: the book's GUID, ALTVCHID 68, ALTMSTID 245, as the run lead's condition asks.
- BK by C5 (capture before Accept 21:21:36Z): Data, Backup & Restore, Backup; Backspace to the path, Specify Path, `C:\lab\2480-nc\backup-1` (Tally showed `C:\lab\2480-Nc\backup-1`); the book the one company; End of List; Accept y; back at the Gateway with no message. One file of 90,051 B, at 02:51 IST, 11 Nov (`evidence/bk.txt`).
- Step 2 (s2): each N read 0 rows; B-bills no group; the snapshot 18 LEDGER rows and no row for any of the eleven; `st.py <the before option>` `FOLD: CLEAR`; `st.py --names` `NAMES: EQUAL`; the marks 68/245 (C5's marks read after the backup).
- Step 3 (21:22:51Z to 21:24:26Z), step 4 (21:25:22Z to 21:28:31Z), step 5, step 6 (21:29:24Z to 21:31:15Z) and step 7 as in Result, each in the notes.
- Step 8 C6 (prep 21:31:45Z: the BK re-hashed equal, its folder holding that file alone, `restore-plan.txt` written, the data folder listed): the book shut by K: Company, Shut, Enter, no prompt; Data, Backup & Restore, Restore; Restore Data Path Tally's data folder; Specify Path, the backup-1 folder; the list showed the book, its number and 11-Nov-26 (2:51 am), as the plan; the book selected alone; Accept ? y (21:33:00Z); the folder-exists question with the book's number (21:33:16Z), r Replace; back at Select Company with no other prompt; the data folder listed again. The book loaded alone by the name filter; Gateway captured, date and period unchanged; then s9.
- Step 9, C10 (21:34:30Z): the book shut by K: Company, Shut, Enter on its one row, no prompt; Select Company with none loaded; one `tally.exe` owning 9001; GET /status status=200, bytes=51. No request after the shut. Tally then quit by its own Quit, no prompt; at 21:35:14Z no `tally.exe` ran and nothing listened on 9001.

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
== a-s2-N-0241.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0242.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0243.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0244.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0245.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0246.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0247.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0248.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0249.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0250.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0251.xml: 3022 bytes
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
  pl 'BRIDGE Nc Plain': 0 groups
  ad 'BRIDGE Nc Ak & Am Pvt. Ltd.': 0 groups
  sh 'BRIDGE Nc M/s Bk-Bm': 0 groups
  bd 'BRIDGE Nc Ck (Cm) Cn.': 0 groups
  qa "BRIDGE Nc Dk's & Dm": 0 groups
  wd 'BRIDGE Nc Ek and Em 2023-24': 0 groups
  ns 'BRIDGE Nc Fk.Fm/Fn-Fp': 0 groups
  sm 'BRIDGE Nc उदाहरण & Gk.': 0 groups
  ob 'BRIDGE Nc (Hk) Hm': 0 groups
  al "BRIDGE Nc Jk-Jm. & Jn/Jp (Jq) Jr's and 2023-24": 0 groups
  rs 'BRIDGE Nc Kk & Sons (India) Pvt. Ltd.': 0 groups
  groups of any other party: 0
== a-s2-S-snap-20260401-20260801.xml
a-s2-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 18
  pl 'BRIDGE Nc Plain': no row
  ad 'BRIDGE Nc Ak & Am Pvt. Ltd.': no row
  sh 'BRIDGE Nc M/s Bk-Bm': no row
  bd 'BRIDGE Nc Ck (Cm) Cn.': no row
  qa "BRIDGE Nc Dk's & Dm": no row
  wd 'BRIDGE Nc Ek and Em 2023-24': no row
  ns 'BRIDGE Nc Fk.Fm/Fn-Fp': no row
  sm 'BRIDGE Nc उदाहरण & Gk.': no row
  ob 'BRIDGE Nc (Hk) Hm': no row
  al "BRIDGE Nc Jk-Jm. & Jn/Jp (Jq) Jr's and 2023-24": no row
  rs 'BRIDGE Nc Kk & Sons (India) Pvt. Ltd.': no row
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
  pl 'BRIDGE Nc Plain': no row
  ad 'BRIDGE Nc Ak & Am Pvt. Ltd.': no row
  sh 'BRIDGE Nc M/s Bk-Bm': no row
  bd 'BRIDGE Nc Ck (Cm) Cn.': no row
  qa "BRIDGE Nc Dk's & Dm": no row
  wd 'BRIDGE Nc Ek and Em 2023-24': no row
  ns 'BRIDGE Nc Fk.Fm/Fn-Fp': no row
  sm 'BRIDGE Nc उदाहरण & Gk.': no row
  ob 'BRIDGE Nc (Hk) Hm': no row
  al "BRIDGE Nc Jk-Jm. & Jn/Jp (Jq) Jr's and 2023-24": no row
  rs 'BRIDGE Nc Kk & Sons (India) Pvt. Ltd.': no row
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
2026-11-10T21:22:55.845Z GET /status after p1: curl 0 "status=200, bytes=51"
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
p2 L-lo-ad-20260801.xml: run.py stopped (log-p2.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T21:23:02.733Z GET /status after p2: curl 0 "status=200, bytes=51"
```

### p2 nm.py
```
== a-p2-L-lo-ad-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p2-L-lo-ad-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p3 empty-answer check
```
p3 L-lo-sh-20260801.xml: run.py stopped (log-p3.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T21:23:10.372Z GET /status after p3: curl 0 "status=200, bytes=51"
```

### p3 nm.py
```
== a-p3-L-lo-sh-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p3-L-lo-sh-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p4 empty-answer check
```
p4 L-lo-bd-20260801.xml: run.py stopped (log-p4.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T21:23:21.549Z GET /status after p4: curl 0 "status=200, bytes=51"
```

### p4 nm.py
```
== a-p4-L-lo-bd-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p4-L-lo-bd-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p5 empty-answer check
```
p5 L-lo-qa-20260801.xml: run.py stopped (log-p5.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T21:23:30.371Z GET /status after p5: curl 0 "status=200, bytes=51"
```

### p5 nm.py
```
== a-p5-L-lo-qa-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p5-L-lo-qa-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p6 empty-answer check
```
p6 L-lo-wd-20260801.xml: run.py stopped (log-p6.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T21:23:39.646Z GET /status after p6: curl 0 "status=200, bytes=51"
```

### p6 nm.py
```
== a-p6-L-lo-wd-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p6-L-lo-wd-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p7 empty-answer check
```
p7 L-lo-ns-20260801.xml: run.py stopped (log-p7.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T21:23:49.080Z GET /status after p7: curl 0 "status=200, bytes=51"
```

### p7 nm.py
```
== a-p7-L-lo-ns-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p7-L-lo-ns-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p8 empty-answer check
```
p8 L-lo-sm-20260801.xml: run.py stopped (log-p8.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T21:23:58.213Z GET /status after p8: curl 0 "status=200, bytes=51"
```

### p8 nm.py
```
== a-p8-L-lo-sm-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p8-L-lo-sm-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p9 empty-answer check
```
p9 L-lo-ob-20260801.xml: run.py stopped (log-p9.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T21:24:07.840Z GET /status after p9: curl 0 "status=200, bytes=51"
```

### p9 nm.py
```
== a-p9-L-lo-ob-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p9-L-lo-ob-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p10 empty-answer check
```
p10 L-lo-al-20260801.xml: run.py stopped (log-p10.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T21:24:17.214Z GET /status after p10: curl 0 "status=200, bytes=51"
```

### p10 nm.py
```
== a-p10-L-lo-al-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p10-L-lo-al-20260801.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### p11 empty-answer check
```
p11 L-lo-rs-20260801.xml: run.py stopped (log-p11.txt: curl 0	0 bytes)
answer file 0 bytes
headers file:
  HTTP/1.1 200 OK
  RESPSTATUS: 1
  CONNECTION: KEEP-ALIVE
  CONTENT-LENGTH: 0
  
EXCEPTION HOLDS: HTTP 200, CONTENT-LENGTH: 0, 0 bytes
2026-11-10T21:24:26.575Z GET /status after p11: curl 0 "status=200, bytes=51"
```

### p11 nm.py
```
== a-p11-L-lo-rs-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-p11-L-lo-rs-20260801.xml: 0 bytes
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

### s4adm count20.py (the before-write option)
```
== a-s4adm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
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
== a-s5adm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'247'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5ad st.py --import
```
a-s5ad-I-ad.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 46, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4shm count20.py (the before-write option)
```
== a-s4shm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'247'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4sh st.py --import
```
a-s4sh-LM-sh.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5shm count20.py (the before-write option)
```
== a-s5shm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5sh st.py --import
```
a-s5sh-I-sh.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 47, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4bdm count20.py (the before-write option)
```
== a-s4bdm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4bd st.py --import
```
a-s4bd-LM-bd.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5bdm count20.py (the before-write option)
```
== a-s5bdm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'249'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5bd st.py --import
```
a-s5bd-I-bd.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 48, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4qam count20.py (the before-write option)
```
== a-s4qam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'249'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
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
== a-s5qam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'250'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5qa st.py --import
```
a-s5qa-I-qa.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 49, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4wdm count20.py (the before-write option)
```
== a-s4wdm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'73'", "'250'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4wd st.py --import
```
a-s4wd-LM-wd.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5wdm count20.py (the before-write option)
```
== a-s5wdm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'73'", "'251'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5wd st.py --import
```
a-s5wd-I-wd.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 50, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4nsm count20.py (the before-write option)
```
== a-s4nsm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'251'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4ns st.py --import
```
a-s4ns-LM-ns.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5nsm count20.py (the before-write option)
```
== a-s5nsm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5ns st.py --import
```
a-s5ns-I-ns.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 51, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4smm count20.py (the before-write option)
```
== a-s4smm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'75'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4sm st.py --import
```
a-s4sm-LM-sm.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5smm count20.py (the before-write option)
```
== a-s5smm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'75'", "'253'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5sm st.py --import
```
a-s5sm-I-sm.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 52, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4obm count20.py (the before-write option)
```
== a-s4obm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'76'", "'253'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4ob st.py --import
```
a-s4ob-LM-ob.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5obm count20.py (the before-write option)
```
== a-s5obm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'76'", "'254'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5ob st.py --import
```
a-s5ob-I-ob.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 53, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4alm count20.py (the before-write option)
```
== a-s4alm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'77'", "'254'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
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
== a-s5alm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'77'", "'255'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5al st.py --import
```
a-s5al-I-al.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 54, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s4rsm count20.py (the before-write option)
```
== a-s4rsm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'78'", "'255'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4rs st.py --import
```
a-s4rs-LM-rs.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### s5rsm count20.py (the before-write option)
```
== a-s5rsm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'78'", "'256'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5rs st.py --import
```
a-s5rs-I-rs.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 55, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
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
  pl 'BRIDGE Nc Plain': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ad 'BRIDGE Nc Ak & Am Pvt. Ltd.': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  sh 'BRIDGE Nc M/s Bk-Bm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  bd 'BRIDGE Nc Ck (Cm) Cn.': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  qa "BRIDGE Nc Dk's & Dm": PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  wd 'BRIDGE Nc Ek and Em 2023-24': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ns 'BRIDGE Nc Fk.Fm/Fn-Fp': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  sm 'BRIDGE Nc उदाहरण & Gk.': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  ob 'BRIDGE Nc (Hk) Hm': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  al "BRIDGE Nc Jk-Jm. & Jn/Jp (Jq) Jr's and 2023-24": PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  rs 'BRIDGE Nc Kk & Sons (India) Pvt. Ltd.': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
exit 0
```

### s6ba nm.py
```
== a-s6ba-B-bills.xml
  BILLFIXED groups: 11
  pl 'BRIDGE Nc Plain': 1 groups
    BILLREF 'BP/26-27/0241'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ad 'BRIDGE Nc Ak & Am Pvt. Ltd.': 1 groups
    BILLREF 'BP/26-27/0242'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  sh 'BRIDGE Nc M/s Bk-Bm': 1 groups
    BILLREF 'BP/26-27/0243'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  bd 'BRIDGE Nc Ck (Cm) Cn.': 1 groups
    BILLREF 'BP/26-27/0244'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  qa "BRIDGE Nc Dk's & Dm": 1 groups
    BILLREF 'BP/26-27/0245'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  wd 'BRIDGE Nc Ek and Em 2023-24': 1 groups
    BILLREF 'BP/26-27/0246'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ns 'BRIDGE Nc Fk.Fm/Fn-Fp': 1 groups
    BILLREF 'BP/26-27/0247'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  sm 'BRIDGE Nc उदाहरण & Gk.': 1 groups
    BILLREF 'BP/26-27/0248'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  ob 'BRIDGE Nc (Hk) Hm': 1 groups
    BILLREF 'BP/26-27/0249'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  al "BRIDGE Nc Jk-Jm. & Jn/Jp (Jq) Jr's and 2023-24": 1 groups
    BILLREF 'BP/26-27/0250'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  rs 'BRIDGE Nc Kk & Sons (India) Pvt. Ltd.': 1 groups
    BILLREF 'BP/26-27/0251'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  groups of any other party: 0
exit 0
```

### s6sa/s6sb nm.py --same
```
  a-s6sa-S-snap-20260401-20260801.xml: 39634 bytes, sha256 08b7edd1249e2d60739c4d41b04c63792fac32ebbf11e98698af9c344de47b42
  a-s6sb-S-snap-20260401-20260801.xml: 39634 bytes, sha256 08b7edd1249e2d60739c4d41b04c63792fac32ebbf11e98698af9c344de47b42
SAME: EQUAL
exit 0
```

### s6ba/s6bb nm.py --same
```
  a-s6ba-B-bills.xml: 5338 bytes, sha256 2cb3a66f5755b24a067cb96281f27e8f92e7d76af569dd9675f625de49e0938c
  a-s6bb-B-bills.xml: 5338 bytes, sha256 2cb3a66f5755b24a067cb96281f27e8f92e7d76af569dd9675f625de49e0938c
SAME: EQUAL
exit 0
```

### s6 st.py --names
```
a-s6sa-S-snap-20260401-20260801.xml: LEDGER rows 29
  pl 'BRIDGE Nc Plain': names ['BRIDGE Nc Plain']
  ad 'BRIDGE Nc Ak & Am Pvt. Ltd.': names ['BRIDGE Nc Ak & Am Pvt. Ltd.']
  sh 'BRIDGE Nc M/s Bk-Bm': names ['BRIDGE Nc M/s Bk-Bm']
  bd 'BRIDGE Nc Ck (Cm) Cn.': names ['BRIDGE Nc Ck (Cm) Cn.']
  qa "BRIDGE Nc Dk's & Dm": names ["BRIDGE Nc Dk's & Dm"]
  wd 'BRIDGE Nc Ek and Em 2023-24': names ['BRIDGE Nc Ek and Em 2023-24']
  ns 'BRIDGE Nc Fk.Fm/Fn-Fp': names ['BRIDGE Nc Fk.Fm/Fn-Fp']
  sm 'BRIDGE Nc उदाहरण & Gk.': names ['BRIDGE Nc उदाहरण & Gk.']
  ob 'BRIDGE Nc (Hk) Hm': names ['BRIDGE Nc (Hk) Hm']
  al "BRIDGE Nc Jk-Jm. & Jn/Jp (Jq) Jr's and 2023-24": names ["BRIDGE Nc Jk-Jm. & Jn/Jp (Jq) Jr's and 2023-24"]
  rs 'BRIDGE Nc Kk & Sons (India) Pvt. Ltd.': names ['BRIDGE Nc Kk & Sons (India) Pvt. Ltd.']
NAMES: EQUAL
exit 0
```

### r1 (pl) nm.py
```
== a-r1a-L-lo-pl-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0241' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r1 nm.py --same
```
  a-r1a-L-lo-pl-20260801.xml: 896 bytes, sha256 e188af84b16d0e5be397e5462ed047dda1be6728821c459ec439a8d9092a9647
  a-r1b-L-lo-pl-20260801.xml: 896 bytes, sha256 e188af84b16d0e5be397e5462ed047dda1be6728821c459ec439a8d9092a9647
SAME: EQUAL
exit 0
```

### r1 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39634']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39634']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0241' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r1 nm.py --against
```
  BILLREF 'BP/26-27/0241' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Nc Plain': 1
    BILLREF 'BP/26-27/0241'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0241): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r2 (ad) nm.py
```
== a-r2a-L-lo-ad-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0242' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r2 nm.py --same
```
  a-r2a-L-lo-ad-20260801.xml: 896 bytes, sha256 8a03774d015efbb3557d708b902eab6a3dc5b733ebdd173e4630da27fd86527b
  a-r2b-L-lo-ad-20260801.xml: 896 bytes, sha256 8a03774d015efbb3557d708b902eab6a3dc5b733ebdd173e4630da27fd86527b
SAME: EQUAL
exit 0
```

### r2 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39634']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39634']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0242' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r2 nm.py --against
```
  BILLREF 'BP/26-27/0242' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Nc Ak & Am Pvt. Ltd.': 1
    BILLREF 'BP/26-27/0242'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0242): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r3 (sh) nm.py
```
== a-r3a-L-lo-sh-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0243' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r3 nm.py --same
```
  a-r3a-L-lo-sh-20260801.xml: 896 bytes, sha256 64aae74e340abe0cab20a8759d9d917268414ecfa6df3600d6123dac6df6fa99
  a-r3b-L-lo-sh-20260801.xml: 896 bytes, sha256 64aae74e340abe0cab20a8759d9d917268414ecfa6df3600d6123dac6df6fa99
SAME: EQUAL
exit 0
```

### r3 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39634']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39634']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0243' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r3 nm.py --against
```
  BILLREF 'BP/26-27/0243' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Nc M/s Bk-Bm': 1
    BILLREF 'BP/26-27/0243'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0243): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r4 (bd) nm.py
```
== a-r4a-L-lo-bd-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0244' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r4 nm.py --same
```
  a-r4a-L-lo-bd-20260801.xml: 896 bytes, sha256 e27f2ea901d6d11d449f0f9ed8ba552878a6a2ec6c15de737733d593236cf2de
  a-r4b-L-lo-bd-20260801.xml: 896 bytes, sha256 e27f2ea901d6d11d449f0f9ed8ba552878a6a2ec6c15de737733d593236cf2de
SAME: EQUAL
exit 0
```

### r4 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39634']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39634']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0244' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r4 nm.py --against
```
  BILLREF 'BP/26-27/0244' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Nc Ck (Cm) Cn.': 1
    BILLREF 'BP/26-27/0244'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0244): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r5 (qa) nm.py
```
== a-r5a-L-lo-qa-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0245' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r5 nm.py --same
```
  a-r5a-L-lo-qa-20260801.xml: 896 bytes, sha256 ffc09691bf324535660b7ca27467250493b2d8a4831a7cc3fc8317d34d430a20
  a-r5b-L-lo-qa-20260801.xml: 896 bytes, sha256 ffc09691bf324535660b7ca27467250493b2d8a4831a7cc3fc8317d34d430a20
SAME: EQUAL
exit 0
```

### r5 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39634']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39634']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0245' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r5 nm.py --against
```
  BILLREF 'BP/26-27/0245' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named "BRIDGE Nc Dk's & Dm": 1
    BILLREF 'BP/26-27/0245'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0245): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r6 (wd) nm.py
```
== a-r6a-L-lo-wd-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0246' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r6 nm.py --same
```
  a-r6a-L-lo-wd-20260801.xml: 896 bytes, sha256 c7c00dcf20d07069cc5e38ca4647e4937071205a7301a2c2a5041be4e33905b1
  a-r6b-L-lo-wd-20260801.xml: 896 bytes, sha256 c7c00dcf20d07069cc5e38ca4647e4937071205a7301a2c2a5041be4e33905b1
SAME: EQUAL
exit 0
```

### r6 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39634']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39634']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0246' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r6 nm.py --against
```
  BILLREF 'BP/26-27/0246' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Nc Ek and Em 2023-24': 1
    BILLREF 'BP/26-27/0246'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0246): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r7 (ns) nm.py
```
== a-r7a-L-lo-ns-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0247' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r7 nm.py --same
```
  a-r7a-L-lo-ns-20260801.xml: 896 bytes, sha256 91abc8feb50f53ae70d7b75685c958b6f208ae6b3fb2c05d2f8e49b145f611c6
  a-r7b-L-lo-ns-20260801.xml: 896 bytes, sha256 91abc8feb50f53ae70d7b75685c958b6f208ae6b3fb2c05d2f8e49b145f611c6
SAME: EQUAL
exit 0
```

### r7 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39634']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39634']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0247' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r7 nm.py --against
```
  BILLREF 'BP/26-27/0247' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Nc Fk.Fm/Fn-Fp': 1
    BILLREF 'BP/26-27/0247'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0247): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r8 (sm) nm.py
```
== a-r8a-L-lo-sm-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0248' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r8 nm.py --same
```
  a-r8a-L-lo-sm-20260801.xml: 896 bytes, sha256 53d5013677dbf904a9fd53ac89531dcecae4c683ac4ba6889f2304c7fd303683
  a-r8b-L-lo-sm-20260801.xml: 896 bytes, sha256 53d5013677dbf904a9fd53ac89531dcecae4c683ac4ba6889f2304c7fd303683
SAME: EQUAL
exit 0
```

### r8 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39634']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39634']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0248' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r8 nm.py --against
```
  BILLREF 'BP/26-27/0248' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Nc उदाहरण & Gk.': 1
    BILLREF 'BP/26-27/0248'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0248): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r9 (ob) nm.py
```
== a-r9a-L-lo-ob-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0249' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r9 nm.py --same
```
  a-r9a-L-lo-ob-20260801.xml: 896 bytes, sha256 3ae1ae3765e0ee2b581abdf27c6087fab9a73efd750e50293eb136650afa6e74
  a-r9b-L-lo-ob-20260801.xml: 896 bytes, sha256 3ae1ae3765e0ee2b581abdf27c6087fab9a73efd750e50293eb136650afa6e74
SAME: EQUAL
exit 0
```

### r9 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39634']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39634']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0249' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r9 nm.py --against
```
  BILLREF 'BP/26-27/0249' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Nc (Hk) Hm': 1
    BILLREF 'BP/26-27/0249'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0249): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r10 (al) nm.py
```
== a-r10a-L-lo-al-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0250' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r10 nm.py --same
```
  a-r10a-L-lo-al-20260801.xml: 896 bytes, sha256 ade384af03dad64a8c69fecc00e1aa54f29683575f18756546d6149fbfbb20a7
  a-r10b-L-lo-al-20260801.xml: 896 bytes, sha256 ade384af03dad64a8c69fecc00e1aa54f29683575f18756546d6149fbfbb20a7
SAME: EQUAL
exit 0
```

### r10 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39634']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39634']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0250' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r10 nm.py --against
```
  BILLREF 'BP/26-27/0250' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named "BRIDGE Nc Jk-Jm. & Jn/Jp (Jq) Jr's and 2023-24": 1
    BILLREF 'BP/26-27/0250'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0250): yes; sum of BILLCL -1051.00, equals the closing
AGAINST: EQUAL (L rows against the groups)
exit 0
```

### r11 (rs) nm.py
```
== a-r11a-L-lo-rs-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0251' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### r11 nm.py --same
```
  a-r11a-L-lo-rs-20260801.xml: 896 bytes, sha256 2914bea6b2ce3285f55849f43a5a725387d6d7a0e3bb09e56807a5e2d2835ab9
  a-r11b-L-lo-rs-20260801.xml: 896 bytes, sha256 2914bea6b2ce3285f55849f43a5a725387d6d7a0e3bb09e56807a5e2d2835ab9
SAME: EQUAL
exit 0
```

### r11 nm.py --outcome
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39634']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['39634']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0251' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot row: PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### r11 nm.py --against
```
  BILLREF 'BP/26-27/0251' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
  Bills Receivable groups named 'BRIDGE Nc Kk & Sons (India) Pvt. Ltd.': 1
    BILLREF 'BP/26-27/0251'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  snapshot CLOSINGBALANCE: '-1051.00'
  rows 1, groups 1; the one bill is this ledger's own (BP/26-27/0251): yes; sum of BILLCL -1051.00, equals the closing
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
== a-s9-N-0241.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0242.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0243.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0244.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0245.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0246.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0247.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0248.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0249.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0250.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0251.xml: 3022 bytes
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
  pl 'BRIDGE Nc Plain': 0 groups
  ad 'BRIDGE Nc Ak & Am Pvt. Ltd.': 0 groups
  sh 'BRIDGE Nc M/s Bk-Bm': 0 groups
  bd 'BRIDGE Nc Ck (Cm) Cn.': 0 groups
  qa "BRIDGE Nc Dk's & Dm": 0 groups
  wd 'BRIDGE Nc Ek and Em 2023-24': 0 groups
  ns 'BRIDGE Nc Fk.Fm/Fn-Fp': 0 groups
  sm 'BRIDGE Nc उदाहरण & Gk.': 0 groups
  ob 'BRIDGE Nc (Hk) Hm': 0 groups
  al "BRIDGE Nc Jk-Jm. & Jn/Jp (Jq) Jr's and 2023-24": 0 groups
  rs 'BRIDGE Nc Kk & Sons (India) Pvt. Ltd.': 0 groups
  groups of any other party: 0
== a-s9-S-snap-20260401-20260801.xml
a-s9-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 18
  pl 'BRIDGE Nc Plain': no row
  ad 'BRIDGE Nc Ak & Am Pvt. Ltd.': no row
  sh 'BRIDGE Nc M/s Bk-Bm': no row
  bd 'BRIDGE Nc Ck (Cm) Cn.': no row
  qa "BRIDGE Nc Dk's & Dm": no row
  wd 'BRIDGE Nc Ek and Em 2023-24': no row
  ns 'BRIDGE Nc Fk.Fm/Fn-Fp': no row
  sm 'BRIDGE Nc उदाहरण & Gk.': no row
  ob 'BRIDGE Nc (Hk) Hm': no row
  al "BRIDGE Nc Jk-Jm. & Jn/Jp (Jq) Jr's and 2023-24": no row
  rs 'BRIDGE Nc Kk & Sons (India) Pvt. Ltd.': no row
exit 0
```

### s9 against s2, byte for byte
```
N-0241.xml 3022 B byte-equal
N-0242.xml 3022 B byte-equal
N-0243.xml 3022 B byte-equal
N-0244.xml 3022 B byte-equal
N-0245.xml 3022 B byte-equal
N-0246.xml 3022 B byte-equal
N-0247.xml 3022 B byte-equal
N-0248.xml 3022 B byte-equal
N-0249.xml 3022 B byte-equal
N-0250.xml 3022 B byte-equal
N-0251.xml 3022 B byte-equal
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
