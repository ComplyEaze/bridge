# Row 2440-ED: #2440's e-invoice day read answered on both books, each pair equal: 3 rows and 127,966 B on the e-invoice book, with Sales 1's three details; 82 rows and 2,809,996 B in about 0.5 s on the dense day

| send | book | request | rows | bytes | curl first byte, s | curl total, s | timed.py start to end, s |
|---|---|---|---|---|---|---|---|
| m1 | BRIDGE EINVOICE LAB | `M-einv.xml` | marks | 3,398 | 0.006 | 0.006 | 0.04 |
| s1 | BRIDGE EINVOICE LAB | `E1-einvoice-day.xml` | 3 | 127,966 | 0.090 | 0.091 | 0.12 |
| s2 | BRIDGE EINVOICE LAB | `E1-einvoice-day.xml` | 3 | 127,966 | 0.089 | 0.090 | 0.12 |
| m2 | BRIDGE EINVOICE LAB | `M-einv.xml` | marks | 3,398 | 0.005 | 0.006 | 0.05 |
| m3 | BRIDGE CORPUS DENSE | `M-dense.xml` | marks | 3,406 | 0.009 | 0.010 | 0.05 |
| s3 | BRIDGE CORPUS DENSE | `E2-dense-day.xml` | 82 | 2,809,996 | 0.477 | 0.490 | 0.51 |
| s4 | BRIDGE CORPUS DENSE | `E2-dense-day.xml` | 82 | 2,809,996 | 0.531 | 0.562 | 0.59 |
| m4 | BRIDGE CORPUS DENSE | `M-dense.xml` | marks | 3,406 | 0.005 | 0.006 | 0.04 |

Run on 10 Nov 2026, 19:47:51Z to 19:53:33Z (01:17 to 01:23 IST, 11 Nov), on TallyPrime single-user edition: the Gateway on 9001, `tally.exe` file version 1.1.7.2 (`evidence/tally-exe.txt`; the Gateway shows no release number, at either load). The sheet was `lab/2440-ed-sheet` (9f29079d), with all 9 files equal to its `SHA256.txt` (sizes too). Two books in turn, each loaded alone, in one sitting with a Tally start of its own: `BRIDGE EINVOICE LAB` (GUID `eb2f5190-292e-4ed8-bfd3-eace839fe699`) and `BRIDGE CORPUS DENSE` (GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`), both synthetic and in the synthetic-uuid registry. READS ONLY: no import, no backup, no restore, no Bridge build; nothing was keyed in Tally but each load and each shut. Each request file's sha256 and size were checked against the sheet's `SHA256.txt` just before its send (`evidence/wall-*.txt`).

Every value below is M, measured by this run, in one run. Every read was answered, each with curl exit 0 and HTTP 200, and no window but the Gateway showed after any send.

## Deviations from the sheet

- **`python` for `python3`.** This PC has no `python3` command. `timed.py` and `count-ed.py` ran as `python -I` (Python 3.10), unchanged, from the row folder, by the helper `scripts/send.sh`, which also checks the request's hash and records the wall clock before and after each send.
- **The read bound.** LAB-COMMON C3's default bound is 2,000,000 B. s3's answer was 2,809,996 B: nothing more was sent until the run lead answered (from s3's answer at 19:51:18Z to s4 at 19:52:10Z). The run lead's answer: for this item the bound is the sheet's 10 MB keep line, for s3 and s4, the other stop rules unchanged. `count-ed.py` had already read s3 by then, as the sheet's step order runs it straight after the send.
- **The start-up company** loaded by itself when Tally started. It was shut (K: Company, Shut, Enter on its one row, no prompt) before the first book was loaded, on the run lead's standing word.
- **The three Bridge processes** that ran before the row (two `bridge_mcp.exe` and `BridgeCommunication.exe`) were left running, where the sheet says to close every other Bridge process first: no process is ended on this PC, on the run lead's standing word. None was called by the row; no Bridge build ran.
- **The headers files of C3** were not written: `timed.py`, the sheet's sender, records curl's exit, the HTTP code, the times and the bytes received, but no headers.
- **The first listing after the quit** failed on its shell quoting (kept local as `evidence/c10-after-quit-first-command-failed.txt`) and was run again from Python.

## Result

**The e-invoice book (s1 and s2).** As the sheet expected:
- 3 rows, all dated 20250403: Sales `1`, Sales `2` and Payment `1`. Every GUID begins with the book's (`-00000001` to `-00000003` after it).
- Sales `1` holds the three details: `IRN` of 64 characters, `IRNACKNO` 112233, `IRNACKDATE` 20250403. Sales `2` and the Payment hold the three tags once each, empty.
- Row bytes: 47,288, 47,132 and 30,484, so 124,904 in rows and 3,062 B of the rest.
- s2 is byte-equal to s1, and `count-ed.py` finds them equal element for element.
- Against row 1378's answer to this request (127,968 B, sha256 `babba160…`): 2 B fewer. Recorded, not a stop: rows 2162-IRN and 2162-IRN9 wrote to this book since and restored it. Which 2 B differ is not established here, as row 1378 kept only that answer's size and hash.
- Each holds 2 GSTIN-shaped values (the book's invented customer GSTIN), so both stay on the lab PC, by size and sha256 (`kept-local.txt`). **So s1 is not on this branch as a file:** the fixture the sheet's Decides names needs either the run lead's word to admit that GSTIN or a copy of s1 with it replaced, which this row does not make.
- Seconds: 0.091 and 0.090 (curl total).

**The dense day (s3 and s4).**
- 82 rows, all Sales, all dated 20250401, none cancelled; every GUID begins with the book's (82 distinct). The three e-invoice tags appear once in every row, all empty. Each row has 2 ledger entry lists.
- **2,809,996 B** per answer: 2,805,986 in rows and 4,010 of the rest. Per row 34,202 to 34,222 B, mean 34,219.
- **Seconds:** s3 0.477 to the first byte and 0.490 in all; s4 0.531 and 0.562.
- s4 is byte-equal to s3 (sha256 `c4f635c6b21df9730b4a06ad5fa6eeeb757b9668d010f159873029a2875c9c7f`), and `count-ed.py` finds them equal element for element.
- Against the transport's limits (32 MiB and 20 s): each answer is about 8.4% of 32 MiB (33,554,432 B), and each took under 3% of 20 s. Each is counted twice for the pair: about 5.6 MB and 1.05 s together.
- No GSTIN-shaped value. Both are pushed whole.

**The marks.** m1 and m2: the e-invoice book's GUID, ALTVCHID 3 and ALTMSTID 215, as the sheet's state; m2 byte-equal to m1. m3 and m4: the dense book's GUID, ALTVCHID 29900 and ALTMSTID 328, as the sheet's state; m4 byte-equal to m3. So each pair was read at unchanged marks.

**The dense book's data folder:** files=20 while shut, files=23 once loaded, files=20 after the shut, as EXPECT (each `evidence/*-folder-count.txt`; the listings stay on the lab PC).

**What this does not say.** Whether #2440's parse accepts s3 (judged on #2440's branch, as the sheet says); any other day or book; the composed path through a built executable.

## Steps
- Step 0 (19:47:51Z): the sheet's files checked. Each book's data folder found once by name and GUID; the excluded GUID is in neither (`evidence/bookcheck-*.txt`). No `tally.exe` ran.
- Tally started from its desktop shortcut (19:47:58Z). The start-up company was shut (deviations). `BRIDGE`, space, `EINVOICE`, space, `LAB` typed in Select Company's filter; Enter only after a screenshot showed the one row left (`s1-select-einv`).
- Gateway: the book alone; period 1-Apr-25 to 31-Mar-26; current date Thursday, 3-Apr-2025; last entry 3-Apr-25; no Educational mode. Step 0 C2 (19:49:24Z): one `tally.exe`, which alone owns 9001; `GET /status` status=200, bytes=51. m1 is the marks read of C2 d.
- m1, s1 (watched: Tally's window captured 2.7 s after the answer, the Gateway, no dialog), s2, m2.
- C10 for the e-invoice book (19:50:29Z): Gateway captured; K: Company, Shut, Enter on `BRIDGE EINVOICE LAB` alone, no prompt; Select Company with none loaded; one `tally.exe` owning 9001; status=200, bytes=51.
- The dense book: folder counted (20), loaded by the name filter (`s2-select-dense`), folder counted (23). Gateway: the book alone; period 1-Apr-25 to 31-Mar-26; current date Tuesday, 31-Mar-2026; last entry 31-Mar-26. Step 0 C2 (19:51:03Z) as above.
- m3, s3 (window captured after it: the Gateway), the hold for the bound (deviations), s4 (window captured after it: the Gateway), m4.
- C10 for the dense book (19:52:58Z): Gateway captured, current date unchanged; K: Company, Shut, Enter on `BRIDGE CORPUS DENSE` alone, no prompt; Select Company with none loaded; one `tally.exe` owning 9001; status=200, bytes=51; folder counted (20). No request after the shut. Tally then quit by its own Quit, no prompt; at 19:53:33Z no `tally.exe` ran and nothing listened on 9001.

## Files
- `answers/`: m1 to m4, s3 and s4, whole, byte for byte. s1 and s2 stay on the lab PC (`kept-local.txt`).
- `logs/`: `timed.py`'s log of each send.
- `evidence/`: notes (UTC, one line per step); `count-ed.py`'s whole output after each step; each send's request check and wall clock; the book checks; the folder counts; `tally.exe`'s file data. Process, port and folder listings stay on the lab PC.
- `scripts/`: the sitting's helpers. `timed.py` and `count-ed.py` ran as the sheet's files, byte for byte, and are not repeated.
- `SCREENSHOTS.txt`: every capture, by name, size and sha256. They stay on the lab PC.

## Outputs, whole

### step 0: BRIDGE EINVOICE LAB's data folder and the excluded GUID
```
BRIDGE EINVOICE LAB: folder <N>, GUID hits 2, excluded GUID hits 0
folders named so: 1
exit 0
```

### step 0: BRIDGE CORPUS DENSE's data folder and the excluded GUID
```
BRIDGE CORPUS DENSE: folder <N>, GUID hits 1, excluded GUID hits 0
folders named so: 1
exit 0
```

### step 0: the dense book's folder, shut
```
2026-11-10T19:47:51Z dense-shut-before: the book's data folder holds files=20, other entries=0
```

### step 0: the e-invoice book's folder, shut
```
2026-11-10T19:47:51Z einv-shut-before: the book's data folder holds files=18, other entries=0
```

### m1: the request check and the wall clock
```
request M-einv.xml: sheet f8b04aca78312e72312e89e1e0e2e5c681ed08f6c2ba611a1d3c8ce363331918 982; file f8b04aca78312e72312e89e1e0e2e5c681ed08f6c2ba611a1d3c8ce363331918 982
wall clock before the send 2026-11-10T19:49:27.741Z
wall clock after the send 2026-11-10T19:49:28.186Z; timed.py exit 0
```

### m1: count-ed.py after it
```
a-m1-M-einv.xml: 3398 bytes, sha256 41aad235aec1eeb84c15d4de0cd7fee953d375d19bc0e8325abaaf0531ee2887
  GSTIN-shaped values: 0
  COMPANY GUIDs 1; the book's: yes; ALTVCHID ['3']; ALTMSTID ['215']
a-m1-M-einv.xml: curl 0, HTTP 200, first byte 0.005639 s, curl total 0.006494 s, wall 0.04 s, 3398 bytes
count exit 0
```

### s1: the request check and the wall clock
```
request E1-einvoice-day.xml: sheet e0702cbb980e35f77e84cf48085824ef7d500d2f674e62d4c93d8d7257e95a71 1794; file e0702cbb980e35f77e84cf48085824ef7d500d2f674e62d4c93d8d7257e95a71 1794
wall clock before the send 2026-11-10T19:49:38.883Z
wall clock after the send 2026-11-10T19:49:39.366Z; timed.py exit 0
```

### s1: count-ed.py after it
```
a-m1-M-einv.xml: 3398 bytes, sha256 41aad235aec1eeb84c15d4de0cd7fee953d375d19bc0e8325abaaf0531ee2887
  GSTIN-shaped values: 0
  COMPANY GUIDs 1; the book's: yes; ALTVCHID ['3']; ALTMSTID ['215']
a-s1-E1-einvoice-day.xml: 127966 bytes, sha256 f03de5417d5ded4660f77f82f005abe602e972efabbd8554a1bd4fb4174248d0
  GSTIN-shaped values: 2
  rows 3
  row bytes: total 124904, the rest of the answer 3062; per row min 30484, max 47288, mean 41635
  BRIDGEVCHISSALES: '<absent>' 3
  VOUCHERTYPENAME: 'Sales' 2; 'Payment' 1
  ISCANCELLED: 'No' 3
  DATE: first 20250403, last 20250403, distinct 1
  IRN: once in 3 rows, not empty in 1
  IRNACKNO: once in 3 rows, not empty in 1
  IRNACKDATE: once in 3 rows, not empty in 1
  Sales-class rows 0: distinct numbers 0, distinct folded 0, rows whose folded number another Sales-class row also has 0
  GUID: absent or empty in 0 rows; beginning with the book's GUID in 3 rows; distinct 3
  ledger entry lists per row: min 2, max 3, total 8
  row 1: 47288 bytes, 36 children, DATE='20250403' VOUCHERNUMBER='1' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x64 IRNACKNO:1x6 IRNACKDATE:1x8, IRNACKNO '112233', IRNACKDATE '20250403', GUID <book's>-00000001
  row 2: 47132 bytes, 36 children, DATE='20250403' VOUCHERNUMBER='2' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000002
  row 3: 30484 bytes, 35 children, DATE='20250403' VOUCHERNUMBER='1' VOUCHERTYPENAME='Payment' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000003
a-m1-M-einv.xml: curl 0, HTTP 200, first byte 0.005639 s, curl total 0.006494 s, wall 0.04 s, 3398 bytes
a-s1-E1-einvoice-day.xml: curl 0, HTTP 200, first byte 0.089698 s, curl total 0.090888 s, wall 0.12 s, 127966 bytes
count exit 0
```

### s2: the request check and the wall clock
```
request E1-einvoice-day.xml: sheet e0702cbb980e35f77e84cf48085824ef7d500d2f674e62d4c93d8d7257e95a71 1794; file e0702cbb980e35f77e84cf48085824ef7d500d2f674e62d4c93d8d7257e95a71 1794
wall clock before the send 2026-11-10T19:49:55.316Z
wall clock after the send 2026-11-10T19:49:55.788Z; timed.py exit 0
```

### m2: the request check and the wall clock
```
request M-einv.xml: sheet f8b04aca78312e72312e89e1e0e2e5c681ed08f6c2ba611a1d3c8ce363331918 982; file f8b04aca78312e72312e89e1e0e2e5c681ed08f6c2ba611a1d3c8ce363331918 982
wall clock before the send 2026-11-10T19:49:56.382Z
wall clock after the send 2026-11-10T19:49:56.864Z; timed.py exit 0
```

### m2: count-ed.py after it (s2 and m2 are in the output after m2)
```
a-m1-M-einv.xml: 3398 bytes, sha256 41aad235aec1eeb84c15d4de0cd7fee953d375d19bc0e8325abaaf0531ee2887
  GSTIN-shaped values: 0
  COMPANY GUIDs 1; the book's: yes; ALTVCHID ['3']; ALTMSTID ['215']
a-m2-M-einv.xml: 3398 bytes, sha256 41aad235aec1eeb84c15d4de0cd7fee953d375d19bc0e8325abaaf0531ee2887
  GSTIN-shaped values: 0
  COMPANY GUIDs 1; the book's: yes; ALTVCHID ['3']; ALTMSTID ['215']
a-s1-E1-einvoice-day.xml: 127966 bytes, sha256 f03de5417d5ded4660f77f82f005abe602e972efabbd8554a1bd4fb4174248d0
  GSTIN-shaped values: 2
  rows 3
  row bytes: total 124904, the rest of the answer 3062; per row min 30484, max 47288, mean 41635
  BRIDGEVCHISSALES: '<absent>' 3
  VOUCHERTYPENAME: 'Sales' 2; 'Payment' 1
  ISCANCELLED: 'No' 3
  DATE: first 20250403, last 20250403, distinct 1
  IRN: once in 3 rows, not empty in 1
  IRNACKNO: once in 3 rows, not empty in 1
  IRNACKDATE: once in 3 rows, not empty in 1
  Sales-class rows 0: distinct numbers 0, distinct folded 0, rows whose folded number another Sales-class row also has 0
  GUID: absent or empty in 0 rows; beginning with the book's GUID in 3 rows; distinct 3
  ledger entry lists per row: min 2, max 3, total 8
  row 1: 47288 bytes, 36 children, DATE='20250403' VOUCHERNUMBER='1' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x64 IRNACKNO:1x6 IRNACKDATE:1x8, IRNACKNO '112233', IRNACKDATE '20250403', GUID <book's>-00000001
  row 2: 47132 bytes, 36 children, DATE='20250403' VOUCHERNUMBER='2' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000002
  row 3: 30484 bytes, 35 children, DATE='20250403' VOUCHERNUMBER='1' VOUCHERTYPENAME='Payment' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000003
a-s2-E1-einvoice-day.xml: 127966 bytes, sha256 f03de5417d5ded4660f77f82f005abe602e972efabbd8554a1bd4fb4174248d0
  GSTIN-shaped values: 2
  rows 3
  row bytes: total 124904, the rest of the answer 3062; per row min 30484, max 47288, mean 41635
  BRIDGEVCHISSALES: '<absent>' 3
  VOUCHERTYPENAME: 'Sales' 2; 'Payment' 1
  ISCANCELLED: 'No' 3
  DATE: first 20250403, last 20250403, distinct 1
  IRN: once in 3 rows, not empty in 1
  IRNACKNO: once in 3 rows, not empty in 1
  IRNACKDATE: once in 3 rows, not empty in 1
  Sales-class rows 0: distinct numbers 0, distinct folded 0, rows whose folded number another Sales-class row also has 0
  GUID: absent or empty in 0 rows; beginning with the book's GUID in 3 rows; distinct 3
  ledger entry lists per row: min 2, max 3, total 8
  row 1: 47288 bytes, 36 children, DATE='20250403' VOUCHERNUMBER='1' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x64 IRNACKNO:1x6 IRNACKDATE:1x8, IRNACKNO '112233', IRNACKDATE '20250403', GUID <book's>-00000001
  row 2: 47132 bytes, 36 children, DATE='20250403' VOUCHERNUMBER='2' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000002
  row 3: 30484 bytes, 35 children, DATE='20250403' VOUCHERNUMBER='1' VOUCHERTYPENAME='Payment' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000003
a-s1-E1-einvoice-day.xml against a-s2-E1-einvoice-day.xml: rows 3 and 3; element for element equal: yes
a-m1-M-einv.xml: curl 0, HTTP 200, first byte 0.005639 s, curl total 0.006494 s, wall 0.04 s, 3398 bytes
a-m2-M-einv.xml: curl 0, HTTP 200, first byte 0.005257 s, curl total 0.006309 s, wall 0.05 s, 3398 bytes
a-s1-E1-einvoice-day.xml: curl 0, HTTP 200, first byte 0.089698 s, curl total 0.090888 s, wall 0.12 s, 127966 bytes
a-s2-E1-einvoice-day.xml: curl 0, HTTP 200, first byte 0.089109 s, curl total 0.090276 s, wall 0.12 s, 127966 bytes
count exit 0
```

### C10 of the e-invoice book: its folder after the shut
```
2026-11-10T19:50:33Z einv-after-shut: the book's data folder holds files=18, other entries=0
```

### step 2: the dense book's folder, shut
```
2026-11-10T19:50:33Z dense-shut: the book's data folder holds files=20, other entries=0
```

### step 2: the dense book's folder, loaded
```
2026-11-10T19:51:02Z dense-loaded: the book's data folder holds files=23, other entries=0
```

### m3: the request check and the wall clock
```
request M-dense.xml: sheet 040ac47754c40a4e2e736f8be2b3f37cac924213ea66c8c4f0074222f9a00280 982; file 040ac47754c40a4e2e736f8be2b3f37cac924213ea66c8c4f0074222f9a00280 982
wall clock before the send 2026-11-10T19:51:06.399Z
wall clock after the send 2026-11-10T19:51:07.065Z; timed.py exit 0
```

### m3: count-ed.py after it
```
a-m1-M-einv.xml: 3398 bytes, sha256 41aad235aec1eeb84c15d4de0cd7fee953d375d19bc0e8325abaaf0531ee2887
  GSTIN-shaped values: 0
  COMPANY GUIDs 1; the book's: yes; ALTVCHID ['3']; ALTMSTID ['215']
a-m2-M-einv.xml: 3398 bytes, sha256 41aad235aec1eeb84c15d4de0cd7fee953d375d19bc0e8325abaaf0531ee2887
  GSTIN-shaped values: 0
  COMPANY GUIDs 1; the book's: yes; ALTVCHID ['3']; ALTMSTID ['215']
a-m3-M-dense.xml: 3406 bytes, sha256 fcc9086dfa112813c447d23e98eb4500c9a1badbe983d3f8d0c0ceff11dc8430
  GSTIN-shaped values: 0
  COMPANY GUIDs 1; the book's: yes; ALTVCHID ['29900']; ALTMSTID ['328']
a-s1-E1-einvoice-day.xml: 127966 bytes, sha256 f03de5417d5ded4660f77f82f005abe602e972efabbd8554a1bd4fb4174248d0
  GSTIN-shaped values: 2
  rows 3
  row bytes: total 124904, the rest of the answer 3062; per row min 30484, max 47288, mean 41635
  BRIDGEVCHISSALES: '<absent>' 3
  VOUCHERTYPENAME: 'Sales' 2; 'Payment' 1
  ISCANCELLED: 'No' 3
  DATE: first 20250403, last 20250403, distinct 1
  IRN: once in 3 rows, not empty in 1
  IRNACKNO: once in 3 rows, not empty in 1
  IRNACKDATE: once in 3 rows, not empty in 1
  Sales-class rows 0: distinct numbers 0, distinct folded 0, rows whose folded number another Sales-class row also has 0
  GUID: absent or empty in 0 rows; beginning with the book's GUID in 3 rows; distinct 3
  ledger entry lists per row: min 2, max 3, total 8
  row 1: 47288 bytes, 36 children, DATE='20250403' VOUCHERNUMBER='1' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x64 IRNACKNO:1x6 IRNACKDATE:1x8, IRNACKNO '112233', IRNACKDATE '20250403', GUID <book's>-00000001
  row 2: 47132 bytes, 36 children, DATE='20250403' VOUCHERNUMBER='2' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000002
  row 3: 30484 bytes, 35 children, DATE='20250403' VOUCHERNUMBER='1' VOUCHERTYPENAME='Payment' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000003
a-s2-E1-einvoice-day.xml: 127966 bytes, sha256 f03de5417d5ded4660f77f82f005abe602e972efabbd8554a1bd4fb4174248d0
  GSTIN-shaped values: 2
  rows 3
  row bytes: total 124904, the rest of the answer 3062; per row min 30484, max 47288, mean 41635
  BRIDGEVCHISSALES: '<absent>' 3
  VOUCHERTYPENAME: 'Sales' 2; 'Payment' 1
  ISCANCELLED: 'No' 3
  DATE: first 20250403, last 20250403, distinct 1
  IRN: once in 3 rows, not empty in 1
  IRNACKNO: once in 3 rows, not empty in 1
  IRNACKDATE: once in 3 rows, not empty in 1
  Sales-class rows 0: distinct numbers 0, distinct folded 0, rows whose folded number another Sales-class row also has 0
  GUID: absent or empty in 0 rows; beginning with the book's GUID in 3 rows; distinct 3
  ledger entry lists per row: min 2, max 3, total 8
  row 1: 47288 bytes, 36 children, DATE='20250403' VOUCHERNUMBER='1' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x64 IRNACKNO:1x6 IRNACKDATE:1x8, IRNACKNO '112233', IRNACKDATE '20250403', GUID <book's>-00000001
  row 2: 47132 bytes, 36 children, DATE='20250403' VOUCHERNUMBER='2' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000002
  row 3: 30484 bytes, 35 children, DATE='20250403' VOUCHERNUMBER='1' VOUCHERTYPENAME='Payment' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000003
a-s1-E1-einvoice-day.xml against a-s2-E1-einvoice-day.xml: rows 3 and 3; element for element equal: yes
a-m1-M-einv.xml: curl 0, HTTP 200, first byte 0.005639 s, curl total 0.006494 s, wall 0.04 s, 3398 bytes
a-m2-M-einv.xml: curl 0, HTTP 200, first byte 0.005257 s, curl total 0.006309 s, wall 0.05 s, 3398 bytes
a-m3-M-dense.xml: curl 0, HTTP 200, first byte 0.009300 s, curl total 0.009987 s, wall 0.05 s, 3406 bytes
a-s1-E1-einvoice-day.xml: curl 0, HTTP 200, first byte 0.089698 s, curl total 0.090888 s, wall 0.12 s, 127966 bytes
a-s2-E1-einvoice-day.xml: curl 0, HTTP 200, first byte 0.089109 s, curl total 0.090276 s, wall 0.12 s, 127966 bytes
count exit 0
```

### s3: the request check and the wall clock
```
request E2-dense-day.xml: sheet 875d4229098d1dbd6fd50f3d035c071d94e828c138f6c7cc28118d353d6e2bfc 1794; file 875d4229098d1dbd6fd50f3d035c071d94e828c138f6c7cc28118d353d6e2bfc 1794
wall clock before the send 2026-11-10T19:51:17.148Z
wall clock after the send 2026-11-10T19:51:18.041Z; timed.py exit 0
```

### s3: count-ed.py after it
```
a-m1-M-einv.xml: 3398 bytes, sha256 41aad235aec1eeb84c15d4de0cd7fee953d375d19bc0e8325abaaf0531ee2887
  GSTIN-shaped values: 0
  COMPANY GUIDs 1; the book's: yes; ALTVCHID ['3']; ALTMSTID ['215']
a-m2-M-einv.xml: 3398 bytes, sha256 41aad235aec1eeb84c15d4de0cd7fee953d375d19bc0e8325abaaf0531ee2887
  GSTIN-shaped values: 0
  COMPANY GUIDs 1; the book's: yes; ALTVCHID ['3']; ALTMSTID ['215']
a-m3-M-dense.xml: 3406 bytes, sha256 fcc9086dfa112813c447d23e98eb4500c9a1badbe983d3f8d0c0ceff11dc8430
  GSTIN-shaped values: 0
  COMPANY GUIDs 1; the book's: yes; ALTVCHID ['29900']; ALTMSTID ['328']
a-s1-E1-einvoice-day.xml: 127966 bytes, sha256 f03de5417d5ded4660f77f82f005abe602e972efabbd8554a1bd4fb4174248d0
  GSTIN-shaped values: 2
  rows 3
  row bytes: total 124904, the rest of the answer 3062; per row min 30484, max 47288, mean 41635
  BRIDGEVCHISSALES: '<absent>' 3
  VOUCHERTYPENAME: 'Sales' 2; 'Payment' 1
  ISCANCELLED: 'No' 3
  DATE: first 20250403, last 20250403, distinct 1
  IRN: once in 3 rows, not empty in 1
  IRNACKNO: once in 3 rows, not empty in 1
  IRNACKDATE: once in 3 rows, not empty in 1
  Sales-class rows 0: distinct numbers 0, distinct folded 0, rows whose folded number another Sales-class row also has 0
  GUID: absent or empty in 0 rows; beginning with the book's GUID in 3 rows; distinct 3
  ledger entry lists per row: min 2, max 3, total 8
  row 1: 47288 bytes, 36 children, DATE='20250403' VOUCHERNUMBER='1' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x64 IRNACKNO:1x6 IRNACKDATE:1x8, IRNACKNO '112233', IRNACKDATE '20250403', GUID <book's>-00000001
  row 2: 47132 bytes, 36 children, DATE='20250403' VOUCHERNUMBER='2' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000002
  row 3: 30484 bytes, 35 children, DATE='20250403' VOUCHERNUMBER='1' VOUCHERTYPENAME='Payment' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000003
a-s2-E1-einvoice-day.xml: 127966 bytes, sha256 f03de5417d5ded4660f77f82f005abe602e972efabbd8554a1bd4fb4174248d0
  GSTIN-shaped values: 2
  rows 3
  row bytes: total 124904, the rest of the answer 3062; per row min 30484, max 47288, mean 41635
  BRIDGEVCHISSALES: '<absent>' 3
  VOUCHERTYPENAME: 'Sales' 2; 'Payment' 1
  ISCANCELLED: 'No' 3
  DATE: first 20250403, last 20250403, distinct 1
  IRN: once in 3 rows, not empty in 1
  IRNACKNO: once in 3 rows, not empty in 1
  IRNACKDATE: once in 3 rows, not empty in 1
  Sales-class rows 0: distinct numbers 0, distinct folded 0, rows whose folded number another Sales-class row also has 0
  GUID: absent or empty in 0 rows; beginning with the book's GUID in 3 rows; distinct 3
  ledger entry lists per row: min 2, max 3, total 8
  row 1: 47288 bytes, 36 children, DATE='20250403' VOUCHERNUMBER='1' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x64 IRNACKNO:1x6 IRNACKDATE:1x8, IRNACKNO '112233', IRNACKDATE '20250403', GUID <book's>-00000001
  row 2: 47132 bytes, 36 children, DATE='20250403' VOUCHERNUMBER='2' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000002
  row 3: 30484 bytes, 35 children, DATE='20250403' VOUCHERNUMBER='1' VOUCHERTYPENAME='Payment' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000003
a-s3-E2-dense-day.xml: 2809996 bytes, sha256 c4f635c6b21df9730b4a06ad5fa6eeeb757b9668d010f159873029a2875c9c7f
  GSTIN-shaped values: 0
  rows 82
  row bytes: total 2805986, the rest of the answer 4010; per row min 34202, max 34222, mean 34219
  BRIDGEVCHISSALES: '<absent>' 82
  VOUCHERTYPENAME: 'Sales' 82
  ISCANCELLED: 'No' 82
  DATE: first 20250401, last 20250401, distinct 1
  IRN: once in 82 rows, not empty in 0
  IRNACKNO: once in 82 rows, not empty in 0
  IRNACKDATE: once in 82 rows, not empty in 0
  Sales-class rows 0: distinct numbers 0, distinct folded 0, rows whose folded number another Sales-class row also has 0
  GUID: absent or empty in 0 rows; beginning with the book's GUID in 82 rows; distinct 82
  ledger entry lists per row: min 2, max 2, total 164
  row 1: 34202 bytes, 35 children, DATE='20250401' VOUCHERNUMBER='1' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000001
  row 2: 34202 bytes, 35 children, DATE='20250401' VOUCHERNUMBER='2' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000002
  row 3: 34202 bytes, 35 children, DATE='20250401' VOUCHERNUMBER='3' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000003
a-s1-E1-einvoice-day.xml against a-s2-E1-einvoice-day.xml: rows 3 and 3; element for element equal: yes
a-m1-M-einv.xml: curl 0, HTTP 200, first byte 0.005639 s, curl total 0.006494 s, wall 0.04 s, 3398 bytes
a-m2-M-einv.xml: curl 0, HTTP 200, first byte 0.005257 s, curl total 0.006309 s, wall 0.05 s, 3398 bytes
a-m3-M-dense.xml: curl 0, HTTP 200, first byte 0.009300 s, curl total 0.009987 s, wall 0.05 s, 3406 bytes
a-s1-E1-einvoice-day.xml: curl 0, HTTP 200, first byte 0.089698 s, curl total 0.090888 s, wall 0.12 s, 127966 bytes
a-s2-E1-einvoice-day.xml: curl 0, HTTP 200, first byte 0.089109 s, curl total 0.090276 s, wall 0.12 s, 127966 bytes
a-s3-E2-dense-day.xml: curl 0, HTTP 200, first byte 0.476745 s, curl total 0.489827 s, wall 0.51 s, 2809996 bytes
count exit 0
```

### s4: the request check and the wall clock
```
request E2-dense-day.xml: sheet 875d4229098d1dbd6fd50f3d035c071d94e828c138f6c7cc28118d353d6e2bfc 1794; file 875d4229098d1dbd6fd50f3d035c071d94e828c138f6c7cc28118d353d6e2bfc 1794
wall clock before the send 2026-11-10T19:52:09.960Z
wall clock after the send 2026-11-10T19:52:11.087Z; timed.py exit 0
```

### s4: count-ed.py after it
```
a-m1-M-einv.xml: 3398 bytes, sha256 41aad235aec1eeb84c15d4de0cd7fee953d375d19bc0e8325abaaf0531ee2887
  GSTIN-shaped values: 0
  COMPANY GUIDs 1; the book's: yes; ALTVCHID ['3']; ALTMSTID ['215']
a-m2-M-einv.xml: 3398 bytes, sha256 41aad235aec1eeb84c15d4de0cd7fee953d375d19bc0e8325abaaf0531ee2887
  GSTIN-shaped values: 0
  COMPANY GUIDs 1; the book's: yes; ALTVCHID ['3']; ALTMSTID ['215']
a-m3-M-dense.xml: 3406 bytes, sha256 fcc9086dfa112813c447d23e98eb4500c9a1badbe983d3f8d0c0ceff11dc8430
  GSTIN-shaped values: 0
  COMPANY GUIDs 1; the book's: yes; ALTVCHID ['29900']; ALTMSTID ['328']
a-s1-E1-einvoice-day.xml: 127966 bytes, sha256 f03de5417d5ded4660f77f82f005abe602e972efabbd8554a1bd4fb4174248d0
  GSTIN-shaped values: 2
  rows 3
  row bytes: total 124904, the rest of the answer 3062; per row min 30484, max 47288, mean 41635
  BRIDGEVCHISSALES: '<absent>' 3
  VOUCHERTYPENAME: 'Sales' 2; 'Payment' 1
  ISCANCELLED: 'No' 3
  DATE: first 20250403, last 20250403, distinct 1
  IRN: once in 3 rows, not empty in 1
  IRNACKNO: once in 3 rows, not empty in 1
  IRNACKDATE: once in 3 rows, not empty in 1
  Sales-class rows 0: distinct numbers 0, distinct folded 0, rows whose folded number another Sales-class row also has 0
  GUID: absent or empty in 0 rows; beginning with the book's GUID in 3 rows; distinct 3
  ledger entry lists per row: min 2, max 3, total 8
  row 1: 47288 bytes, 36 children, DATE='20250403' VOUCHERNUMBER='1' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x64 IRNACKNO:1x6 IRNACKDATE:1x8, IRNACKNO '112233', IRNACKDATE '20250403', GUID <book's>-00000001
  row 2: 47132 bytes, 36 children, DATE='20250403' VOUCHERNUMBER='2' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000002
  row 3: 30484 bytes, 35 children, DATE='20250403' VOUCHERNUMBER='1' VOUCHERTYPENAME='Payment' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000003
a-s2-E1-einvoice-day.xml: 127966 bytes, sha256 f03de5417d5ded4660f77f82f005abe602e972efabbd8554a1bd4fb4174248d0
  GSTIN-shaped values: 2
  rows 3
  row bytes: total 124904, the rest of the answer 3062; per row min 30484, max 47288, mean 41635
  BRIDGEVCHISSALES: '<absent>' 3
  VOUCHERTYPENAME: 'Sales' 2; 'Payment' 1
  ISCANCELLED: 'No' 3
  DATE: first 20250403, last 20250403, distinct 1
  IRN: once in 3 rows, not empty in 1
  IRNACKNO: once in 3 rows, not empty in 1
  IRNACKDATE: once in 3 rows, not empty in 1
  Sales-class rows 0: distinct numbers 0, distinct folded 0, rows whose folded number another Sales-class row also has 0
  GUID: absent or empty in 0 rows; beginning with the book's GUID in 3 rows; distinct 3
  ledger entry lists per row: min 2, max 3, total 8
  row 1: 47288 bytes, 36 children, DATE='20250403' VOUCHERNUMBER='1' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x64 IRNACKNO:1x6 IRNACKDATE:1x8, IRNACKNO '112233', IRNACKDATE '20250403', GUID <book's>-00000001
  row 2: 47132 bytes, 36 children, DATE='20250403' VOUCHERNUMBER='2' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000002
  row 3: 30484 bytes, 35 children, DATE='20250403' VOUCHERNUMBER='1' VOUCHERTYPENAME='Payment' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000003
a-s3-E2-dense-day.xml: 2809996 bytes, sha256 c4f635c6b21df9730b4a06ad5fa6eeeb757b9668d010f159873029a2875c9c7f
  GSTIN-shaped values: 0
  rows 82
  row bytes: total 2805986, the rest of the answer 4010; per row min 34202, max 34222, mean 34219
  BRIDGEVCHISSALES: '<absent>' 82
  VOUCHERTYPENAME: 'Sales' 82
  ISCANCELLED: 'No' 82
  DATE: first 20250401, last 20250401, distinct 1
  IRN: once in 82 rows, not empty in 0
  IRNACKNO: once in 82 rows, not empty in 0
  IRNACKDATE: once in 82 rows, not empty in 0
  Sales-class rows 0: distinct numbers 0, distinct folded 0, rows whose folded number another Sales-class row also has 0
  GUID: absent or empty in 0 rows; beginning with the book's GUID in 82 rows; distinct 82
  ledger entry lists per row: min 2, max 2, total 164
  row 1: 34202 bytes, 35 children, DATE='20250401' VOUCHERNUMBER='1' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000001
  row 2: 34202 bytes, 35 children, DATE='20250401' VOUCHERNUMBER='2' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000002
  row 3: 34202 bytes, 35 children, DATE='20250401' VOUCHERNUMBER='3' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000003
a-s4-E2-dense-day.xml: 2809996 bytes, sha256 c4f635c6b21df9730b4a06ad5fa6eeeb757b9668d010f159873029a2875c9c7f
  GSTIN-shaped values: 0
  rows 82
  row bytes: total 2805986, the rest of the answer 4010; per row min 34202, max 34222, mean 34219
  BRIDGEVCHISSALES: '<absent>' 82
  VOUCHERTYPENAME: 'Sales' 82
  ISCANCELLED: 'No' 82
  DATE: first 20250401, last 20250401, distinct 1
  IRN: once in 82 rows, not empty in 0
  IRNACKNO: once in 82 rows, not empty in 0
  IRNACKDATE: once in 82 rows, not empty in 0
  Sales-class rows 0: distinct numbers 0, distinct folded 0, rows whose folded number another Sales-class row also has 0
  GUID: absent or empty in 0 rows; beginning with the book's GUID in 82 rows; distinct 82
  ledger entry lists per row: min 2, max 2, total 164
  row 1: 34202 bytes, 35 children, DATE='20250401' VOUCHERNUMBER='1' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000001
  row 2: 34202 bytes, 35 children, DATE='20250401' VOUCHERNUMBER='2' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000002
  row 3: 34202 bytes, 35 children, DATE='20250401' VOUCHERNUMBER='3' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000003
a-s1-E1-einvoice-day.xml against a-s2-E1-einvoice-day.xml: rows 3 and 3; element for element equal: yes
a-s3-E2-dense-day.xml against a-s4-E2-dense-day.xml: rows 82 and 82; element for element equal: yes
a-m1-M-einv.xml: curl 0, HTTP 200, first byte 0.005639 s, curl total 0.006494 s, wall 0.04 s, 3398 bytes
a-m2-M-einv.xml: curl 0, HTTP 200, first byte 0.005257 s, curl total 0.006309 s, wall 0.05 s, 3398 bytes
a-m3-M-dense.xml: curl 0, HTTP 200, first byte 0.009300 s, curl total 0.009987 s, wall 0.05 s, 3406 bytes
a-s1-E1-einvoice-day.xml: curl 0, HTTP 200, first byte 0.089698 s, curl total 0.090888 s, wall 0.12 s, 127966 bytes
a-s2-E1-einvoice-day.xml: curl 0, HTTP 200, first byte 0.089109 s, curl total 0.090276 s, wall 0.12 s, 127966 bytes
a-s3-E2-dense-day.xml: curl 0, HTTP 200, first byte 0.476745 s, curl total 0.489827 s, wall 0.51 s, 2809996 bytes
a-s4-E2-dense-day.xml: curl 0, HTTP 200, first byte 0.530902 s, curl total 0.561972 s, wall 0.59 s, 2809996 bytes
count exit 0
```

### m4: the request check and the wall clock
```
request M-dense.xml: sheet 040ac47754c40a4e2e736f8be2b3f37cac924213ea66c8c4f0074222f9a00280 982; file 040ac47754c40a4e2e736f8be2b3f37cac924213ea66c8c4f0074222f9a00280 982
wall clock before the send 2026-11-10T19:52:25.360Z
wall clock after the send 2026-11-10T19:52:25.835Z; timed.py exit 0
```

### m4: count-ed.py after it
```
a-m1-M-einv.xml: 3398 bytes, sha256 41aad235aec1eeb84c15d4de0cd7fee953d375d19bc0e8325abaaf0531ee2887
  GSTIN-shaped values: 0
  COMPANY GUIDs 1; the book's: yes; ALTVCHID ['3']; ALTMSTID ['215']
a-m2-M-einv.xml: 3398 bytes, sha256 41aad235aec1eeb84c15d4de0cd7fee953d375d19bc0e8325abaaf0531ee2887
  GSTIN-shaped values: 0
  COMPANY GUIDs 1; the book's: yes; ALTVCHID ['3']; ALTMSTID ['215']
a-m3-M-dense.xml: 3406 bytes, sha256 fcc9086dfa112813c447d23e98eb4500c9a1badbe983d3f8d0c0ceff11dc8430
  GSTIN-shaped values: 0
  COMPANY GUIDs 1; the book's: yes; ALTVCHID ['29900']; ALTMSTID ['328']
a-m4-M-dense.xml: 3406 bytes, sha256 fcc9086dfa112813c447d23e98eb4500c9a1badbe983d3f8d0c0ceff11dc8430
  GSTIN-shaped values: 0
  COMPANY GUIDs 1; the book's: yes; ALTVCHID ['29900']; ALTMSTID ['328']
a-s1-E1-einvoice-day.xml: 127966 bytes, sha256 f03de5417d5ded4660f77f82f005abe602e972efabbd8554a1bd4fb4174248d0
  GSTIN-shaped values: 2
  rows 3
  row bytes: total 124904, the rest of the answer 3062; per row min 30484, max 47288, mean 41635
  BRIDGEVCHISSALES: '<absent>' 3
  VOUCHERTYPENAME: 'Sales' 2; 'Payment' 1
  ISCANCELLED: 'No' 3
  DATE: first 20250403, last 20250403, distinct 1
  IRN: once in 3 rows, not empty in 1
  IRNACKNO: once in 3 rows, not empty in 1
  IRNACKDATE: once in 3 rows, not empty in 1
  Sales-class rows 0: distinct numbers 0, distinct folded 0, rows whose folded number another Sales-class row also has 0
  GUID: absent or empty in 0 rows; beginning with the book's GUID in 3 rows; distinct 3
  ledger entry lists per row: min 2, max 3, total 8
  row 1: 47288 bytes, 36 children, DATE='20250403' VOUCHERNUMBER='1' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x64 IRNACKNO:1x6 IRNACKDATE:1x8, IRNACKNO '112233', IRNACKDATE '20250403', GUID <book's>-00000001
  row 2: 47132 bytes, 36 children, DATE='20250403' VOUCHERNUMBER='2' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000002
  row 3: 30484 bytes, 35 children, DATE='20250403' VOUCHERNUMBER='1' VOUCHERTYPENAME='Payment' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000003
a-s2-E1-einvoice-day.xml: 127966 bytes, sha256 f03de5417d5ded4660f77f82f005abe602e972efabbd8554a1bd4fb4174248d0
  GSTIN-shaped values: 2
  rows 3
  row bytes: total 124904, the rest of the answer 3062; per row min 30484, max 47288, mean 41635
  BRIDGEVCHISSALES: '<absent>' 3
  VOUCHERTYPENAME: 'Sales' 2; 'Payment' 1
  ISCANCELLED: 'No' 3
  DATE: first 20250403, last 20250403, distinct 1
  IRN: once in 3 rows, not empty in 1
  IRNACKNO: once in 3 rows, not empty in 1
  IRNACKDATE: once in 3 rows, not empty in 1
  Sales-class rows 0: distinct numbers 0, distinct folded 0, rows whose folded number another Sales-class row also has 0
  GUID: absent or empty in 0 rows; beginning with the book's GUID in 3 rows; distinct 3
  ledger entry lists per row: min 2, max 3, total 8
  row 1: 47288 bytes, 36 children, DATE='20250403' VOUCHERNUMBER='1' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x64 IRNACKNO:1x6 IRNACKDATE:1x8, IRNACKNO '112233', IRNACKDATE '20250403', GUID <book's>-00000001
  row 2: 47132 bytes, 36 children, DATE='20250403' VOUCHERNUMBER='2' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000002
  row 3: 30484 bytes, 35 children, DATE='20250403' VOUCHERNUMBER='1' VOUCHERTYPENAME='Payment' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000003
a-s3-E2-dense-day.xml: 2809996 bytes, sha256 c4f635c6b21df9730b4a06ad5fa6eeeb757b9668d010f159873029a2875c9c7f
  GSTIN-shaped values: 0
  rows 82
  row bytes: total 2805986, the rest of the answer 4010; per row min 34202, max 34222, mean 34219
  BRIDGEVCHISSALES: '<absent>' 82
  VOUCHERTYPENAME: 'Sales' 82
  ISCANCELLED: 'No' 82
  DATE: first 20250401, last 20250401, distinct 1
  IRN: once in 82 rows, not empty in 0
  IRNACKNO: once in 82 rows, not empty in 0
  IRNACKDATE: once in 82 rows, not empty in 0
  Sales-class rows 0: distinct numbers 0, distinct folded 0, rows whose folded number another Sales-class row also has 0
  GUID: absent or empty in 0 rows; beginning with the book's GUID in 82 rows; distinct 82
  ledger entry lists per row: min 2, max 2, total 164
  row 1: 34202 bytes, 35 children, DATE='20250401' VOUCHERNUMBER='1' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000001
  row 2: 34202 bytes, 35 children, DATE='20250401' VOUCHERNUMBER='2' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000002
  row 3: 34202 bytes, 35 children, DATE='20250401' VOUCHERNUMBER='3' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000003
a-s4-E2-dense-day.xml: 2809996 bytes, sha256 c4f635c6b21df9730b4a06ad5fa6eeeb757b9668d010f159873029a2875c9c7f
  GSTIN-shaped values: 0
  rows 82
  row bytes: total 2805986, the rest of the answer 4010; per row min 34202, max 34222, mean 34219
  BRIDGEVCHISSALES: '<absent>' 82
  VOUCHERTYPENAME: 'Sales' 82
  ISCANCELLED: 'No' 82
  DATE: first 20250401, last 20250401, distinct 1
  IRN: once in 82 rows, not empty in 0
  IRNACKNO: once in 82 rows, not empty in 0
  IRNACKDATE: once in 82 rows, not empty in 0
  Sales-class rows 0: distinct numbers 0, distinct folded 0, rows whose folded number another Sales-class row also has 0
  GUID: absent or empty in 0 rows; beginning with the book's GUID in 82 rows; distinct 82
  ledger entry lists per row: min 2, max 2, total 164
  row 1: 34202 bytes, 35 children, DATE='20250401' VOUCHERNUMBER='1' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000001
  row 2: 34202 bytes, 35 children, DATE='20250401' VOUCHERNUMBER='2' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000002
  row 3: 34202 bytes, 35 children, DATE='20250401' VOUCHERNUMBER='3' VOUCHERTYPENAME='Sales' ISCANCELLED='No' BRIDGEVCHISSALES=<absent>, IRN:1x0 IRNACKNO:1x0 IRNACKDATE:1x0, IRNACKNO '<0 chars>', IRNACKDATE '', GUID <book's>-00000003
a-s1-E1-einvoice-day.xml against a-s2-E1-einvoice-day.xml: rows 3 and 3; element for element equal: yes
a-s3-E2-dense-day.xml against a-s4-E2-dense-day.xml: rows 82 and 82; element for element equal: yes
a-m1-M-einv.xml: curl 0, HTTP 200, first byte 0.005639 s, curl total 0.006494 s, wall 0.04 s, 3398 bytes
a-m2-M-einv.xml: curl 0, HTTP 200, first byte 0.005257 s, curl total 0.006309 s, wall 0.05 s, 3398 bytes
a-m3-M-dense.xml: curl 0, HTTP 200, first byte 0.009300 s, curl total 0.009987 s, wall 0.05 s, 3406 bytes
a-m4-M-dense.xml: curl 0, HTTP 200, first byte 0.005173 s, curl total 0.005706 s, wall 0.04 s, 3406 bytes
a-s1-E1-einvoice-day.xml: curl 0, HTTP 200, first byte 0.089698 s, curl total 0.090888 s, wall 0.12 s, 127966 bytes
a-s2-E1-einvoice-day.xml: curl 0, HTTP 200, first byte 0.089109 s, curl total 0.090276 s, wall 0.12 s, 127966 bytes
a-s3-E2-dense-day.xml: curl 0, HTTP 200, first byte 0.476745 s, curl total 0.489827 s, wall 0.51 s, 2809996 bytes
a-s4-E2-dense-day.xml: curl 0, HTTP 200, first byte 0.530902 s, curl total 0.561972 s, wall 0.59 s, 2809996 bytes
count exit 0
```

### C10 of the dense book: its folder after the shut
```
2026-11-10T19:53:02Z dense-after-shut: the book's data folder holds files=20, other entries=0
```

### the program file of the Tally that answered
```
$ the program file of the Tally that answered (read after C10, with Tally closed)
tally.exe file version 1.1.7.2; 18485952 B; sha256 856e5236a7664c0d8d580585e1c31b443720badd7a0078512d41f6c380b7ed73; last written 2026-10-25T05:34:45Z
```

### size and sha256 of every answer
```
$ size and sha256 of every answer as saved on the lab PC (Tally's answers, as timed.py wrote them)
a-m1-M-einv.xml 3398 B 41aad235aec1eeb84c15d4de0cd7fee953d375d19bc0e8325abaaf0531ee2887
a-m2-M-einv.xml 3398 B 41aad235aec1eeb84c15d4de0cd7fee953d375d19bc0e8325abaaf0531ee2887
a-m3-M-dense.xml 3406 B fcc9086dfa112813c447d23e98eb4500c9a1badbe983d3f8d0c0ceff11dc8430
a-m4-M-dense.xml 3406 B fcc9086dfa112813c447d23e98eb4500c9a1badbe983d3f8d0c0ceff11dc8430
a-s1-E1-einvoice-day.xml 127966 B f03de5417d5ded4660f77f82f005abe602e972efabbd8554a1bd4fb4174248d0
a-s2-E1-einvoice-day.xml 127966 B f03de5417d5ded4660f77f82f005abe602e972efabbd8554a1bd4fb4174248d0
a-s3-E2-dense-day.xml 2809996 B c4f635c6b21df9730b4a06ad5fa6eeeb757b9668d010f159873029a2875c9c7f
a-s4-E2-dense-day.xml 2809996 B c4f635c6b21df9730b4a06ad5fa6eeeb757b9668d010f159873029a2875c9c7f
```

## timed.py's logs (each send: UTC start, UTC end, request, answer, curl exit, then HTTP code, seconds to the first byte, seconds in all, bytes)

### m1
```
2026-11-10T19:49:28.024Z	2026-11-10T19:49:28.068Z	M-einv.xml	a-m1-M-einv.xml	curl 0	200 0.005639 0.006494 3398
```

### s1
```
2026-11-10T19:49:39.166Z	2026-11-10T19:49:39.284Z	E1-einvoice-day.xml	a-s1-E1-einvoice-day.xml	curl 0	200 0.089698 0.090888 127966
```

### s2
```
2026-11-10T19:49:55.588Z	2026-11-10T19:49:55.705Z	E1-einvoice-day.xml	a-s2-E1-einvoice-day.xml	curl 0	200 0.089109 0.090276 127966
```

### m2
```
2026-11-10T19:49:56.682Z	2026-11-10T19:49:56.731Z	M-einv.xml	a-m2-M-einv.xml	curl 0	200 0.005257 0.006309 3398
```

### m3
```
2026-11-10T19:51:06.898Z	2026-11-10T19:51:06.944Z	M-dense.xml	a-m3-M-dense.xml	curl 0	200 0.009300 0.009987 3406
```

### s3
```
2026-11-10T19:51:17.432Z	2026-11-10T19:51:17.941Z	E2-dense-day.xml	a-s3-E2-dense-day.xml	curl 0	200 0.476745 0.489827 2809996
```

### s4
```
2026-11-10T19:52:10.347Z	2026-11-10T19:52:10.942Z	E2-dense-day.xml	a-s4-E2-dense-day.xml	curl 0	200 0.530902 0.561972 2809996
```

### m4
```
2026-11-10T19:52:25.668Z	2026-11-10T19:52:25.709Z	M-dense.xml	a-m4-M-dense.xml	curl 0	200 0.005173 0.005706 3406
```

