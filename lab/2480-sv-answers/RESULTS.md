# Row 2480-SV: each of the six vouchers landed on the ledger it names; `kb`'s voucher, naming the altered ledger's other spelling, landed on that ledger, `ka`'s, and every scoped read after held its own vouchers and nothing else

Run on 11 Nov 2026, 07:32:54Z to 08:00:29Z (13:02 to 13:30 IST), on TallyPrime single-user edition: the Gateway on 9001, `tally.exe` file version 1.1.7.2 (`evidence/tally-exe.txt`, equal to row 2480-SC's record; the Gateway shows no release number). The sheet was `lab/2480-sv-sheet` (c3fedc88), with all 99 files equal to its `SHA256.txt`. One book, `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic, in the synthetic-uuid registry), loaded alone. No Bridge build ran and no Bridge tool was called. Each request file's sha256 and size were checked against the sheet's `SHA256.txt` just before its send (`evidence/sends-utc.txt`).

**No `WRONG LANDING:` line.** Every `sv.py --landed` verdict was `ON ITS LEDGER` (exit 0), so step 7 passed in full and step 8 was sent.

**Confidence: M, measured by this run, in one run, for each line below; PARTIAL as the sheet's Decides says (one book, one release, one sitting, raw requests sent by curl, without the reads Bridge sends around them and without the build's own parse).** Every send was answered: each with curl exit 0 and HTTP 200, with bytes; no answer was empty, so exception (a) was never needed. No import was refused, so `rf.py` and exception (b) were never used. Exception (c) gave the one ALTERED result, at `s4kb`, as the sheet builds on. No dialog, message or prompt showed but C5's and C6's own. Every capture taken after a send (after each import, each catalogue read, s2, s9 and each send of steps 4 to 8 but m1 and the marks reads before a write, which had none) was the Gateway, one of the images in `evidence/gateway-refs.txt` (carried from row 2480-SC).

## The set-up (steps 1 to 5)

**Step 1.** The Gateway: the book alone; period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026 (neither changed in the row). s1's marks were 68/245, as the sheet asks, before the backup. BK by C5: one file of 90,051 B, at 13:05 IST, 11 Nov (`evidence/bk.txt`).

**Step 2, the book before.** Each of the seventeen N reads (N-0281, N-0292 to N-0301, N-0303 to N-0308) 0 rows; B-bills no group for any of the eleven names; the snapshot 18 LEDGER rows and "no row" for each name; `st.py` (the before option) `FOLD: CLEAR`; `st.py --names` `NAMES: EQUAL`; `cv.py` on the book-wide census `ANSWER: VOUCHERS 11`; `sk.py --names` on the catalogue 18 LEDGER rows and "no row" for each of the eleven; the marks 68/245. **The sixteen answers row 2480-SC also sent were byte-equal to SC's s2** (the six new N reads are this row's own). Each answer's sha256 is in `evidence/s2-sha256.txt`.

**Step 3, SC's set-up.** Each write by SC's recipe, each after a marks read that passed the before-write check (14 checks, each exit 0). Every answer was the one SC's sheet names:

| part | answer | the result |
| --- | --- | --- |
| the control `pl` | `s4pl`, `s5pl` each `IMPORT: CREATED ONE` (`LASTVCHID` 45 at `s5pl`) | CREATED ONE twice |
| pair `k` | `s4ka`, `s5ka` CREATED ONE (`LASTVCHID` 46); `s4kb` CREATED 0, ALTERED 1, the rest 0, no LINEERROR; recipe (e): `sk.py --altered` ALTERED ONE, s4kbx 70/248 with the before-write check passed; s7kb: `sc.py --altered-state` 20 LEDGER rows | **ALTERED STATE k: AS SK RECORDED** |
| pair `q` | `s4qa`, `s5qa` (`LASTVCHID` 47), `s4qb` CREATED ONE; s7qb: `sk.py --pair qa qb` 22 LEDGER rows, both `EQUAL`; then `s5qb` CREATED ONE (`LASTVCHID` 48) | **BOTH** |
| pair `u` | `s4ua`, `s5ua` (`LASTVCHID` 49), `s4ub` CREATED ONE; s7ub: `sk.py --pair ua ub` 24 LEDGER rows, both `EQUAL`; then `s5ub` CREATED ONE (`LASTVCHID` 50) | **BOTH** |

On the alteration, `count20.py --same-row` of s4kbx against s4kbm (70/247) printed that the COMPANY row differs (ALTMSTID one higher), exit 1: SC's sheet makes that line a record, not a check.

**Step 4, the book-wide reads.** s6m 74/252. The snapshot, sent twice, byte-equal: 24 LEDGER rows; a row for each of `pl`, `ka`, `qa`, `qb`, `ua` and `ub`, each bill-wise Yes, closing -1051.00; "no row" for `kb`. B-bills, sent twice, byte-equal: 6 groups, one for each of those six names (`BP/26-27/0281`, `0292`, `0296`, `0297`, `0298`, `0299`), `ka`'s under `ka`'s name; none for `kb`. The catalogue, sent twice, byte-equal: `sk.py --names` `EQUAL` for `pl`, `qa`, `qb`, `ua` and `ub`, `NAMES ka: DIFFER (its names are ['BRIDGE Sk Ak, Am'])`, `NAMES kb: no row`; `sc.py --altered-state` `ALTERED STATE k: AS SK RECORDED`. The census, sent twice, byte-equal: `ANSWER: VOUCHERS 17`. The outputs of `nm.py` on s6sa and s6ba, `sk.py --names` and `sc.py --altered-state` on s6ga, `st.py --names` on s6sa and `count20.py` on s6m were byte-equal to row 2480-SC's; the census held the same seventeen vouchers as SC's.

**Step 5.** m1's marks were equal to s6m's (74/252). No m1r was sent.

## The vouchers (steps 6 and 7)

**Before v0** (written in `evidence/notes.txt` before v0, each with the line that shows it): no stop at steps 1 to 5; every send answered; m1 equal to s6m; every before-write check passed. All four held.

**Step 6.** v0: the book-wide read `ANSWER: VOUCHERS 17`, SC's k8 count (1,361,516 B). k0: the marks equal to m1's (74/252).

**Step 7.** Each voucher by hand, one label a command: its marks read and before-write check (each passed, ALTVCHID 74 to 79 in turn, ALTMSTID 252 throughout), its import alone, a capture of the window (the Gateway each time), `st.py --import`, then the book-wide read and `sv.py --landed` against the read before it:

| n | id | number | import | the book-wide read after | `sv.py --landed` | result |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | qa | 0303 | CREATED ONE, `LASTVCHID` 51 | v1: VOUCHERS 18, 1,453,442 B | `LANDED qa: ON qa`; `VERDICT qa: ON ITS LEDGER` | **LANDED ON ITS LEDGER** |
| 2 | qb | 0304 | CREATED ONE, `LASTVCHID` 52 | v2: VOUCHERS 19, 1,545,392 B | `LANDED qb: ON qb`; `VERDICT qb: ON ITS LEDGER` | **LANDED ON ITS LEDGER** |
| 3 | ua | 0305 | CREATED ONE, `LASTVCHID` 53 | v3: VOUCHERS 20, 1,637,322 B | `LANDED ua: ON ua`; `VERDICT ua: ON ITS LEDGER` | **LANDED ON ITS LEDGER** |
| 4 | ub | 0306 | CREATED ONE, `LASTVCHID` 54 | v4: VOUCHERS 21, 1,729,256 B | `LANDED ub: ON ub`; `VERDICT ub: ON ITS LEDGER` | **LANDED ON ITS LEDGER** |
| 5 | ka | 0307 | CREATED ONE, `LASTVCHID` 55 | v5: VOUCHERS 22, 1,821,184 B | `LANDED ka: ON ka`; `VERDICT ka: ON ITS LEDGER` | **LANDED ON ITS LEDGER** |
| 6 | kb | 0308 | CREATED ONE, `LASTVCHID` 56 | v6: VOUCHERS 23, 1,913,112 B | `LANDED kb: ON ka`; `VERDICT kb: ON ITS LEDGER` | **LANDED ON ITS LEDGER** |

Each import's counters were CREATED 1, the rest 0, no LINEERROR. `kb`'s target is the altered ledger, whose `NAME` attribute is `ka`'s and whose name list holds `kb`'s spelling (step 4): its voucher, naming `BRIDGE Sk Ak, Am`, landed there. Neither pair's voucher landed on its sibling. **Step 7 passed in full**: every voucher LANDED ON ITS LEDGER, every send answered, no dialog. Each book-wide read was within the sheet's 2,000,000 B bound; the last, v6, held 1,913,112 B.

## The reads after (step 8)

k1's marks were 80/252: against k0 (74/252) `count20.py --same-row` printed that the COMPANY row differs, exit 1, a record the sheet names (the six vouchers moved the voucher mark). BOOK was v6. Each id by `scripts/r8.sh`, one id a run, each send once, each answer read by its reader before the next request, the window captured after every send:

| id | census (`C-v-`) | movement (`Vm-v-`) | window (`Vw-v-`) | outstandings (`L-lo-`) | each pair |
| --- | --- | --- | --- | --- | --- |
| qa | VOUCHERS 2; `JUDGE2 qa census: OWN 2 OF 2, OTHERS 0` | VOUCHERS 2; `OWN 2 OF 2, OTHERS 0` | VOUCHERS 2; `OWN 2 OF 2, OTHERS 0` | not sent | SAME: EQUAL |
| qb | VOUCHERS 2; `OWN 2 OF 2, OTHERS 0` | VOUCHERS 2; `OWN 2 OF 2, OTHERS 0` | VOUCHERS 2; `OWN 2 OF 2, OTHERS 0` | not sent | SAME: EQUAL |
| ua | VOUCHERS 2; `OWN 2 OF 2, OTHERS 0` | VOUCHERS 2; `OWN 2 OF 2, OTHERS 0` | VOUCHERS 2; `OWN 2 OF 2, OTHERS 0` | not sent | SAME: EQUAL |
| ub | VOUCHERS 2; `OWN 2 OF 2, OTHERS 0` | VOUCHERS 2; `OWN 2 OF 2, OTHERS 0` | VOUCHERS 2; `OWN 2 OF 2, OTHERS 0` | not sent | SAME: EQUAL |
| ka | VOUCHERS 3; `OWN 3 OF 3, OTHERS 0` | VOUCHERS 3; `OWN 3 OF 3, OTHERS 0` | VOUCHERS 3; `OWN 3 OF 3, OTHERS 0` | ROWS 3 (0 On Account); `OWN 3 OF 3, OTHERS 0` | SAME: EQUAL |
| kb | VOUCHERS 3; `OWN 3 OF 3, OTHERS 0` | VOUCHERS 3; `OWN 3 OF 3, OTHERS 0` | VOUCHERS 3; `OWN 3 OF 3, OTHERS 0` | ROWS 3 (0 On Account); `OWN 3 OF 3, OTHERS 0` | SAME: EQUAL |

Every `JUDGE2` line exited 0. For each of `qa` to `ub`, the two vouchers are its first invoice and this row's voucher. For `ka` and `kb`, the three are `ka`'s first invoice and the two vouchers of this row that landed on the altered ledger. **`kb`'s four answers were byte-equal to `ka`'s** (compared with `cmp`): the altered ledger read by either spelling. e1: the book-wide read byte-equal to BOOK (`wx.py --same`, 1,913,112 B each), so the book did not change during step 8. e2: the marks equal to k1's (80/252): step 8 only read.

**Against the sheet's Expected.** Steps 1 to 5 gave SC's answers. Step 7: `qa`, `ua` and `ka` CREATED ONE and LANDED ON ITS LEDGER, as estimated; `qb` and `ub`, for which no estimate was made, landed on their own ledgers, not their siblings'; `kb`, for which no estimate was made, was not refused and landed on the altered ledger. Step 8: `OWN 2 OF 2, OTHERS 0` for each of `qa` to `ub`, as estimated; for `ka` and `kb` `<k>` was 3 (`kb`'s voucher landed). s9 equal to s2 and s1, as estimated.

### Deviations from the sheet

- `python -I` for `python3 -I` (the lab PC has no `python3` command), as the sheet allows.
- The run lead's words (a), (b) and (c), given before the row: (c) gave the one ALTERED result at `s4kb`; (a) was not needed; (b) was not used.
- Tally was not started for this row: it was running at Select Company with no company loaded, left so by the previous item's C10 (step 0 found one `tally.exe` owning 9001), and C10 left it so again. It was not quit.
- The three Bridge processes were left running, as before the row.
- Sends from step 2 on ran in the background beside a 300 s watch (`scripts/bg.sh`, `scripts/wd.sh`); every run ended within it.
- Step 3 ran one part a command (the control, then each pair), by SC's `ctl.sh` and `pair.sh`; step 8 ran one id a command by a new helper, `scripts/r8.sh`, which captured the window after every send, not only after each `a` send.
- One line of `evidence/notes.txt` counted the before-write files as 17; the next line corrects it to 14 (the notes are only appended to).
- The first `cmp` of `kb`'s answers against `ka`'s named the wrong files and compared nothing; it was run again on the exact names, and those results are the ones given above.
- In the lab's own text a 16-hex sha256 head holding a long digit run is replaced by its answer's whole sha256, on the run lead's word (`substitutions.txt`).

### Not measured

What Bridge itself would post or read: no Bridge build ran. A second release, a second book, or a second sitting. Whether Tally would land the same vouchers the same way after a restart.

## The way back

**Step 9.** BK was restored by C6, with only the two expected prompts. Then s9 sent every file of s2, in s2's order: every answer byte-equal to its s2 answer (`evidence/s9-against-s2.txt`, 22 of 22), and the marks equal to s1's (68/245; `count20.py --same-row`). So the book holds again what it held at step 1.

**The side files.** Before the row, four of the seven side files were in the book's data folder (20 entries). At load, before step 1's reads, 23 entries. Before the restore the folder held 24 entries; the restore left out all seven (17 entries after), and at the load after it six were back and `Index.1800` was absent (23 entries), as in rows 2480-NX to 2480-SC. The listings stay on the lab PC.

**Step 10.** C10 (the steps below).

**Timings.** Every read sent without a window capture after it (the marks reads before each write) returned within 1 s of its send line (0.433 to 0.817 s), wall clock, python's start included. The labels sent with a capture took 3.174 to 6.348 s, the capture included: the set-up imports 3.584 to 4.858 s, step 3's catalogue reads 3.331 to 3.993 s, step 4's reads 3.440 to 5.551 s, the vouchers' imports 3.792 to 4.226 s, the book-wide reads v0 to v6 3.598 to 4.571 s, step 8's sends 3.326 to 4.553 s, and s2 and s9 (twenty-two reads each) 6.337 and 6.348 s (`evidence/durations.txt`).

## Steps
- Step 0 (07:32:54Z): the sheet's files checked; the book's data folder found once by name and GUID; the excluded GUID absent (`evidence/bookcheck.txt`). The backup folder made new and empty. One `tally.exe` at Select Company with no company loaded, which alone owned 9001; GET /status status=200, bytes=51.
- `BRIDGE`, space, `PILOT`, space, `LAB` typed in Select Company's filter; Enter only after a capture showed the one row left (`s1-select`, 07:33:20Z, byte-equal to row 2480-SC's).
- Gateway: the book alone; period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026. s1 (07:33:40Z): the book's GUID, ALTVCHID 68, ALTMSTID 245.
- BK by C5: Data, Backup & Restore, Backup; Specify Path, `C:\lab\2480-sv\backup-1`; the book the one company; End of List; Accept y; back at the Gateway with no message (07:35:10Z).
- Step 2 (07:35:46Z), step 3 (07:37:56Z to 07:40:20Z), step 4 (07:40:45Z to 07:41:29Z), step 5 (07:42:13Z), step 6 (07:42:50Z to 07:43:14Z), step 7 (07:43:31Z to 07:48:09Z) and step 8 (07:48:45Z to 07:55:22Z) as above, each in the notes.
- Step 9 C6 (prep 07:56:08Z: the BK re-hashed equal, its folder holding that file alone, `restore-plan.txt` written, the data folder listed): the book shut by K: Company, Shut, Enter (a click on the title bar first), no prompt; Data, Backup & Restore, Restore; Restore Data Path Tally's data folder; the list opened at the drive's root; Specify Path, the backup-1 folder; the list showed the book, its number and 11-Nov-26 (1:05 pm), as the plan; the book selected alone; End of List; Accept ? y (07:58:06Z); the folder-exists question with the book's number (07:58:27Z), Replace; back at Select Company with no other prompt. The book loaded alone by the name filter (`s9-select`, 07:58:56Z, the one row, byte-equal to `s1-select`); Enter, no prompt; Gateway captured (07:59:11Z), one of the list, date and period unchanged; then s9 (07:59:28Z to 07:59:35Z), equal to s2 throughout.
- Step 10, C10 (08:00:29Z): the book shut by K: Company, Shut, Enter on its one row, no prompt; Select Company with none loaded (the capture byte-equal to step 0's); one `tally.exe` owning 9001; GET /status status=200, bytes=51. No request after the shut. The BK, listed again, one file, its sha256 equal to `evidence/bk.txt`.

## Files
- `answers/`: every answer and its headers file, whole, byte for byte (`kept-local.txt` lists any that stay on the lab PC).
- `logs/`: `run.py`'s log of each label.
- `evidence/`: notes (UTC, one line per step); every `count20.py`, `nm.py`, `st.py`, `cv.py`, `wx.py`, `sk.py`, `sc.py` and `sv.py` output, whole, with its exit; each `er.sh` run's output; each send's UTC time and request check; each background run's output and exit; the Gateway images' list; the per-label wall clock; the book check; `tally.exe`'s file data. Process, port, folder and backup listings and the restore plan stay on the lab PC.
- `scripts/`: the sitting's helpers. `run.py`, `count20.py`, `nm.py`, `st.py`, `rf.py`, `wx.py`, `cv.py`, `sk.py`, `sc.py` and `sv.py` are the sheet's files, byte for byte, and are not repeated.
- `SCREENSHOTS.txt`: every capture, by name, size and sha256. They stay on the lab PC.
- `substitutions.txt`: where the lab's own text differs from the files kept on the lab PC.

## The outputs, whole

### evidence/count-s1.txt
```
== a-s1-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/count-s2.txt
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
== a-s2-N-0303.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0304.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0305.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0306.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0307.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0308.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
exit 0
```

### evidence/nm-s2.txt
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

### evidence/cv-s2.txt
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

### evidence/names-s2.txt
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

### evidence/st-before-s2.txt
```
a-s2-S-snap-20260401-20260801.xml: LEDGER rows 18
FOLD: CLEAR
exit 0
```

### evidence/st-names-s2.txt
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

### evidence/s2-sha256.txt
```
8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3 *a-s2-B-bills.xml
1cd8c03cae154deef7eb72f68d3d73b5c757d3bfe517907e750b31fe8d4ee0a0 *a-s2-C-book.xml
d05d2af351d78027720df3f9e95116f41bd14b9b1f82ea95fcc50764dd2cd205 *a-s2-G-cat.xml
8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6 *a-s2-M-marks.xml
947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 *a-s2-N-0281.xml
947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 *a-s2-N-0292.xml
947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 *a-s2-N-0293.xml
947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 *a-s2-N-0294.xml
947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 *a-s2-N-0295.xml
947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 *a-s2-N-0296.xml
947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 *a-s2-N-0297.xml
947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 *a-s2-N-0298.xml
947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 *a-s2-N-0299.xml
947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 *a-s2-N-0300.xml
947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 *a-s2-N-0301.xml
947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 *a-s2-N-0303.xml
947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 *a-s2-N-0304.xml
947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 *a-s2-N-0305.xml
947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 *a-s2-N-0306.xml
947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 *a-s2-N-0307.xml
947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 *a-s2-N-0308.xml
ffe23742ab0793221f0136c5e119bde7d2e8fd2181f01e0f31520f61608eab69 *a-s2-S-snap-20260401-20260801.xml
```

### evidence/count-s4plm.txt
```
== a-s4plm-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/bw-s4plm.txt
```
== a-s4plm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### evidence/import-s4pl.txt
```
a-s4pl-LM-pl.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### evidence/count-s5plm.txt
```
== a-s5plm-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/bw-s5plm.txt
```
== a-s5plm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### evidence/import-s5pl.txt
```
a-s5pl-I-pl.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 45, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### evidence/count-s4kam.txt
```
== a-s4kam-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/bw-s4kam.txt
```
== a-s4kam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### evidence/import-s4ka.txt
```
a-s4ka-LM-ka.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### evidence/count-s5kam.txt
```
== a-s5kam-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '247'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/bw-s5kam.txt
```
== a-s5kam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'247'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### evidence/import-s5ka.txt
```
a-s5ka-I-ka.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 46, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### evidence/count-s4kbm.txt
```
== a-s4kbm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '247'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/bw-s4kbm.txt
```
== a-s4kbm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'247'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### evidence/import-s4kb.txt
```
a-s4kb-LM-kb.xml: CREATED 0, ALTERED 1, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: stop (CREATED is not 1; ALTERED is not 0)
exit 1
```

### evidence/alt-s4kb.txt
```
a-s4kb-LM-kb.xml: CREATED 0, ALTERED 1, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
ALTERED ONE
exit 0
```

### evidence/count-s4kbx.txt
```
== a-s4kbx-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '248'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/bw-s4kbx.txt
```
== a-s4kbx-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### evidence/same-s4kbx.txt
```
== a-s4kbx-M-marks.xml against a-s4kbm-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'247'")
stop: the COMPANY row differs
exit 1
```

### evidence/names-s7kb.txt
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

### evidence/altstate-s7kb.txt
```
a-s7kb-G-cat.xml: LEDGER rows 20
ALTERED STATE k: AS SK RECORDED
exit 0
```

### evidence/count-s4qam.txt
```
== a-s4qam-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '248'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/bw-s4qam.txt
```
== a-s4qam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### evidence/import-s4qa.txt
```
a-s4qa-LM-qa.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### evidence/count-s5qam.txt
```
== a-s5qam-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '249'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/bw-s5qam.txt
```
== a-s5qam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'249'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### evidence/import-s5qa.txt
```
a-s5qa-I-qa.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 47, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### evidence/count-s4qbm.txt
```
== a-s4qbm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '71' ALTMSTID '249'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/bw-s4qbm.txt
```
== a-s4qbm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'249'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### evidence/import-s4qb.txt
```
a-s4qb-LM-qb.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### evidence/names-s7qb.txt
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

### evidence/pair-s7qb.txt
```
a-s7qb-G-cat.xml: LEDGER rows 22
  NAMES qa: EQUAL (one row, its own GUID)
  NAMES qb: EQUAL (one row, its own GUID)
PAIR qa qb: BOTH
exit 0
```

### evidence/count-s5qbm.txt
```
== a-s5qbm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '71' ALTMSTID '250'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/bw-s5qbm.txt
```
== a-s5qbm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'250'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### evidence/import-s5qb.txt
```
a-s5qb-I-qb.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 48, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### evidence/count-s4uam.txt
```
== a-s4uam-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '72' ALTMSTID '250'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/bw-s4uam.txt
```
== a-s4uam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'250'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### evidence/import-s4ua.txt
```
a-s4ua-LM-ua.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### evidence/count-s5uam.txt
```
== a-s5uam-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '72' ALTMSTID '251'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/bw-s5uam.txt
```
== a-s5uam-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'251'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### evidence/import-s5ua.txt
```
a-s5ua-I-ua.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 49, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### evidence/count-s4ubm.txt
```
== a-s4ubm-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '73' ALTMSTID '251'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/bw-s4ubm.txt
```
== a-s4ubm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'73'", "'251'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### evidence/import-s4ub.txt
```
a-s4ub-LM-ub.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### evidence/names-s7ub.txt
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

### evidence/pair-s7ub.txt
```
a-s7ub-G-cat.xml: LEDGER rows 24
  NAMES ua: EQUAL (one row, its own GUID)
  NAMES ub: EQUAL (one row, its own GUID)
PAIR ua ub: BOTH
exit 0
```

### evidence/count-s5ubm.txt
```
== a-s5ubm-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '73' ALTMSTID '252'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/bw-s5ubm.txt
```
== a-s5ubm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'73'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### evidence/import-s5ub.txt
```
a-s5ub-I-ub.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 50, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### evidence/count-s6m.txt
```
== a-s6m-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '74' ALTMSTID '252'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/nm-s6sa.txt
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

### evidence/nm-s6sb.txt
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

### evidence/same-s6s.txt
```
  a-s6sa-S-snap-20260401-20260801.xml: 32928 bytes, sha256 30198747e1e9b92a0f0fda8b864ff20760db96561f9ed0a639fcd6263f8b5cc5
  a-s6sb-S-snap-20260401-20260801.xml: 32928 bytes, sha256 30198747e1e9b92a0f0fda8b864ff20760db96561f9ed0a639fcd6263f8b5cc5
SAME: EQUAL
exit 0
```

### evidence/nm-s6ba.txt
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

### evidence/nm-s6bb.txt
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

### evidence/same-s6b.txt
```
  a-s6ba-B-bills.xml: 2798 bytes, sha256 db384bc41d5cbdde20f5d21bc6e5bc5dbd66d26e18ebb516153dd333e3fe6745
  a-s6bb-B-bills.xml: 2798 bytes, sha256 db384bc41d5cbdde20f5d21bc6e5bc5dbd66d26e18ebb516153dd333e3fe6745
SAME: EQUAL
exit 0
```

### evidence/names-s6ga.txt
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

### evidence/altstate-s6ga.txt
```
a-s6ga-G-cat.xml: LEDGER rows 24
ALTERED STATE k: AS SK RECORDED
exit 0
```

### evidence/names-s6gb.txt
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

### evidence/same-s6g.txt
```
  a-s6ga-G-cat.xml: 29706 bytes, sha256 797def7057568d629821a33cc69f3a0cb79a5ac9057b2ceac60655f3c64a69a1
  a-s6gb-G-cat.xml: 29706 bytes, sha256 797def7057568d629821a33cc69f3a0cb79a5ac9057b2ceac60655f3c64a69a1
SAME: EQUAL
exit 0
```

### evidence/cv-s6ka.txt
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

### evidence/cv-s6kb.txt
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

### evidence/same-s6k.txt
```
  a-s6ka-C-book.xml: 44458 bytes, sha256 a29f9e255571363cdce35f6346e909d9f28f13d326abb9020213e9525707d43c
  a-s6kb-C-book.xml: 44458 bytes, sha256 a29f9e255571363cdce35f6346e909d9f28f13d326abb9020213e9525707d43c
SAME: EQUAL
exit 0
```

### evidence/st-names-s6.txt
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

### evidence/count-m1.txt
```
== a-m1-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '74' ALTMSTID '252'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/same-m1.txt
```
== a-m1-M-marks.xml against a-s6m-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'252'")
  equal
exit 0
```

### evidence/wx-v0.txt
```
a-v0-Vb-book.xml: ANSWER: VOUCHERS 17
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

### evidence/count-k0.txt
```
== a-k0-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '74' ALTMSTID '252'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/same-k0.txt
```
== a-k0-M-marks.xml against a-m1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'252'")
  equal
exit 0
```

### evidence/count-j1m.txt
```
== a-j1m-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '74' ALTMSTID '252'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/bw-j1m.txt
```
== a-j1m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### evidence/import-j1.txt
```
a-j1-J-qa.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 51, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### evidence/wx-v1.txt
```
a-v1-Vb-book.xml: ANSWER: VOUCHERS 18
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
  BP/26-27/0303 DATE 20260801 ALTERID 75 entries 6
exit 0
```

### evidence/landed-v1.txt
```
LANDED qa: ON qa
VERDICT qa: ON ITS LEDGER
exit 0
```

### evidence/count-j2m.txt
```
== a-j2m-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '75' ALTMSTID '252'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/bw-j2m.txt
```
== a-j2m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'75'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### evidence/import-j2.txt
```
a-j2-J-qb.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 52, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### evidence/wx-v2.txt
```
a-v2-Vb-book.xml: ANSWER: VOUCHERS 19
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
  BP/26-27/0303 DATE 20260801 ALTERID 75 entries 6
  BP/26-27/0304 DATE 20260801 ALTERID 76 entries 6
exit 0
```

### evidence/landed-v2.txt
```
LANDED qb: ON qb
VERDICT qb: ON ITS LEDGER
exit 0
```

### evidence/count-j3m.txt
```
== a-j3m-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '76' ALTMSTID '252'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/bw-j3m.txt
```
== a-j3m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'76'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### evidence/import-j3.txt
```
a-j3-J-ua.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 53, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### evidence/wx-v3.txt
```
a-v3-Vb-book.xml: ANSWER: VOUCHERS 20
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
  BP/26-27/0303 DATE 20260801 ALTERID 75 entries 6
  BP/26-27/0304 DATE 20260801 ALTERID 76 entries 6
  BP/26-27/0305 DATE 20260801 ALTERID 77 entries 6
exit 0
```

### evidence/landed-v3.txt
```
LANDED ua: ON ua
VERDICT ua: ON ITS LEDGER
exit 0
```

### evidence/count-j4m.txt
```
== a-j4m-M-marks.xml: 3400 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '77' ALTMSTID '252'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/bw-j4m.txt
```
== a-j4m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'77'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### evidence/import-j4.txt
```
a-j4-J-ub.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 54, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### evidence/wx-v4.txt
```
a-v4-Vb-book.xml: ANSWER: VOUCHERS 21
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
  BP/26-27/0303 DATE 20260801 ALTERID 75 entries 6
  BP/26-27/0304 DATE 20260801 ALTERID 76 entries 6
  BP/26-27/0305 DATE 20260801 ALTERID 77 entries 6
  BP/26-27/0306 DATE 20260801 ALTERID 78 entries 6
exit 0
```

### evidence/landed-v4.txt
```
LANDED ub: ON ub
VERDICT ub: ON ITS LEDGER
exit 0
```

### evidence/count-j5m.txt
```
== a-j5m-M-marks.xml: 3406 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '78' ALTMSTID '252'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/bw-j5m.txt
```
== a-j5m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'78'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### evidence/import-j5.txt
```
a-j5-J-ka.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 55, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### evidence/wx-v5.txt
```
a-v5-Vb-book.xml: ANSWER: VOUCHERS 22
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
  BP/26-27/0303 DATE 20260801 ALTERID 75 entries 6
  BP/26-27/0304 DATE 20260801 ALTERID 76 entries 6
  BP/26-27/0305 DATE 20260801 ALTERID 77 entries 6
  BP/26-27/0306 DATE 20260801 ALTERID 78 entries 6
  BP/26-27/0307 DATE 20260801 ALTERID 79 entries 6
exit 0
```

### evidence/landed-v5.txt
```
LANDED ka: ON ka
VERDICT ka: ON ITS LEDGER
exit 0
```

### evidence/count-j6m.txt
```
== a-j6m-M-marks.xml: 3406 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '79' ALTMSTID '252'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/bw-j6m.txt
```
== a-j6m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'79'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### evidence/import-j6.txt
```
a-j6-J-kb.xml: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 56, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LINEERROR 0
IMPORT: CREATED ONE
exit 0
```

### evidence/wx-v6.txt
```
a-v6-Vb-book.xml: ANSWER: VOUCHERS 23
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
  BP/26-27/0303 DATE 20260801 ALTERID 75 entries 6
  BP/26-27/0304 DATE 20260801 ALTERID 76 entries 6
  BP/26-27/0305 DATE 20260801 ALTERID 77 entries 6
  BP/26-27/0306 DATE 20260801 ALTERID 78 entries 6
  BP/26-27/0307 DATE 20260801 ALTERID 79 entries 6
  BP/26-27/0308 DATE 20260801 ALTERID 80 entries 6
exit 0
```

### evidence/landed-v6.txt
```
LANDED kb: ON ka
VERDICT kb: ON ITS LEDGER
exit 0
```

### evidence/count-k1.txt
```
== a-k1-M-marks.xml: 3406 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '80' ALTMSTID '252'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/same-k1.txt
```
== a-k1-M-marks.xml against a-k0-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'80'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'252'")
stop: the COMPANY row differs
exit 1
```

### evidence/cv-r1ca.txt
```
a-r1ca-C-v-qa.xml: ANSWER: VOUCHERS 2
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f ALTERID 71 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000033 ALTERID 75 DATE 20260801
exit 0
```

### evidence/cv-r1cb.txt
```
a-r1cb-C-v-qa.xml: ANSWER: VOUCHERS 2
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f ALTERID 71 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000033 ALTERID 75 DATE 20260801
exit 0
```

### evidence/same-r1c.txt
```
SAME: EQUAL (7822 B sha256 bc60a6424d2bd794; 7822 B sha256 bc60a6424d2bd794)
exit 0
```

### evidence/wx-r1qa.txt
```
a-r1qa-Vm-v-qa.xml: ANSWER: VOUCHERS 2
  BP/26-27/0296 DATE 20260801 ALTERID 71 entries 6
  BP/26-27/0303 DATE 20260801 ALTERID 75 entries 6
exit 0
```

### evidence/wx-r1qb.txt
```
a-r1qb-Vm-v-qa.xml: ANSWER: VOUCHERS 2
  BP/26-27/0296 DATE 20260801 ALTERID 71 entries 6
  BP/26-27/0303 DATE 20260801 ALTERID 75 entries 6
exit 0
```

### evidence/same-r1q.txt
```
SAME: EQUAL (20966 B sha256 6eab493fc8e73399; 20966 B sha256 6eab493fc8e73399)
exit 0
```

### evidence/wx-r1wa.txt
```
a-r1wa-Vw-v-qa.xml: ANSWER: VOUCHERS 2
  BP/26-27/0296 DATE 20260801 ALTERID 71 entries 6
  BP/26-27/0303 DATE 20260801 ALTERID 75 entries 6
exit 0
```

### evidence/wx-r1wb.txt
```
a-r1wb-Vw-v-qa.xml: ANSWER: VOUCHERS 2
  BP/26-27/0296 DATE 20260801 ALTERID 71 entries 6
  BP/26-27/0303 DATE 20260801 ALTERID 75 entries 6
exit 0
```

### evidence/same-r1w.txt
```
SAME: EQUAL (186562 B sha256 d705cccd48789752; 186562 B sha256 d705cccd48789752)
exit 0
```

### evidence/judge2-r1c.txt
```
JUDGE2 qa census: OWN 2 OF 2, OTHERS 0
exit 0
```

### evidence/judge2-r1q.txt
```
JUDGE2 qa movement: OWN 2 OF 2, OTHERS 0
exit 0
```

### evidence/judge2-r1w.txt
```
JUDGE2 qa window: OWN 2 OF 2, OTHERS 0
exit 0
```

### evidence/cv-r2ca.txt
```
a-r2ca-C-v-qb.xml: ANSWER: VOUCHERS 2
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000030 ALTERID 72 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000034 ALTERID 76 DATE 20260801
exit 0
```

### evidence/cv-r2cb.txt
```
a-r2cb-C-v-qb.xml: ANSWER: VOUCHERS 2
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000030 ALTERID 72 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000034 ALTERID 76 DATE 20260801
exit 0
```

### evidence/same-r2c.txt
```
SAME: EQUAL (7822 B sha256 cad0ec3388976d67; 7822 B sha256 cad0ec3388976d67)
exit 0
```

### evidence/wx-r2qa.txt
```
a-r2qa-Vm-v-qb.xml: ANSWER: VOUCHERS 2
  BP/26-27/0297 DATE 20260801 ALTERID 72 entries 6
  BP/26-27/0304 DATE 20260801 ALTERID 76 entries 6
exit 0
```

### evidence/wx-r2qb.txt
```
a-r2qb-Vm-v-qb.xml: ANSWER: VOUCHERS 2
  BP/26-27/0297 DATE 20260801 ALTERID 72 entries 6
  BP/26-27/0304 DATE 20260801 ALTERID 76 entries 6
exit 0
```

### evidence/same-r2q.txt
```
SAME: EQUAL (21014 B sha256 bd7eb80d1cb8c266; 21014 B sha256 bd7eb80d1cb8c266)
exit 0
```

### evidence/wx-r2wa.txt
```
a-r2wa-Vw-v-qb.xml: ANSWER: VOUCHERS 2
  BP/26-27/0297 DATE 20260801 ALTERID 72 entries 6
  BP/26-27/0304 DATE 20260801 ALTERID 76 entries 6
exit 0
```

### evidence/wx-r2wb.txt
```
a-r2wb-Vw-v-qb.xml: ANSWER: VOUCHERS 2
  BP/26-27/0297 DATE 20260801 ALTERID 72 entries 6
  BP/26-27/0304 DATE 20260801 ALTERID 76 entries 6
exit 0
```

### evidence/same-r2w.txt
```
SAME: EQUAL (186610 B sha256 190575f692da6167; 186610 B sha256 190575f692da6167)
exit 0
```

### evidence/judge2-r2c.txt
```
JUDGE2 qb census: OWN 2 OF 2, OTHERS 0
exit 0
```

### evidence/judge2-r2q.txt
```
JUDGE2 qb movement: OWN 2 OF 2, OTHERS 0
exit 0
```

### evidence/judge2-r2w.txt
```
JUDGE2 qb window: OWN 2 OF 2, OTHERS 0
exit 0
```

### evidence/cv-r3ca.txt
```
a-r3ca-C-v-ua.xml: ANSWER: VOUCHERS 2
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000031 ALTERID 73 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000035 ALTERID 77 DATE 20260801
exit 0
```

### evidence/cv-r3cb.txt
```
a-r3cb-C-v-ua.xml: ANSWER: VOUCHERS 2
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000031 ALTERID 73 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000035 ALTERID 77 DATE 20260801
exit 0
```

### evidence/same-r3c.txt
```
SAME: EQUAL (7822 B sha256 c703b1aa189f254a; 7822 B sha256 c703b1aa189f254a)
exit 0
```

### evidence/wx-r3qa.txt
```
a-r3qa-Vm-v-ua.xml: ANSWER: VOUCHERS 2
  BP/26-27/0298 DATE 20260801 ALTERID 73 entries 6
  BP/26-27/0305 DATE 20260801 ALTERID 77 entries 6
exit 0
```

### evidence/wx-r3qb.txt
```
a-r3qb-Vm-v-ua.xml: ANSWER: VOUCHERS 2
  BP/26-27/0298 DATE 20260801 ALTERID 73 entries 6
  BP/26-27/0305 DATE 20260801 ALTERID 77 entries 6
exit 0
```

### evidence/same-r3q.txt
```
SAME: EQUAL (20966 B sha256 02e3a378af80ba29; 20966 B sha256 02e3a378af80ba29)
exit 0
```

### evidence/wx-r3wa.txt
```
a-r3wa-Vw-v-ua.xml: ANSWER: VOUCHERS 2
  BP/26-27/0298 DATE 20260801 ALTERID 73 entries 6
  BP/26-27/0305 DATE 20260801 ALTERID 77 entries 6
exit 0
```

### evidence/wx-r3wb.txt
```
a-r3wb-Vw-v-ua.xml: ANSWER: VOUCHERS 2
  BP/26-27/0298 DATE 20260801 ALTERID 73 entries 6
  BP/26-27/0305 DATE 20260801 ALTERID 77 entries 6
exit 0
```

### evidence/same-r3w.txt
```
SAME: EQUAL (186564 B sha256 9ee7bb58e4c6f70c; 186564 B sha256 9ee7bb58e4c6f70c)
exit 0
```

### evidence/judge2-r3c.txt
```
JUDGE2 ua census: OWN 2 OF 2, OTHERS 0
exit 0
```

### evidence/judge2-r3q.txt
```
JUDGE2 ua movement: OWN 2 OF 2, OTHERS 0
exit 0
```

### evidence/judge2-r3w.txt
```
JUDGE2 ua window: OWN 2 OF 2, OTHERS 0
exit 0
```

### evidence/cv-r4ca.txt
```
a-r4ca-C-v-ub.xml: ANSWER: VOUCHERS 2
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000032 ALTERID 74 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000036 ALTERID 78 DATE 20260801
exit 0
```

### evidence/cv-r4cb.txt
```
a-r4cb-C-v-ub.xml: ANSWER: VOUCHERS 2
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000032 ALTERID 74 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000036 ALTERID 78 DATE 20260801
exit 0
```

### evidence/same-r4c.txt
```
SAME: EQUAL (7822 B sha256 1ceb9cea80b0bc2a; 7822 B sha256 1ceb9cea80b0bc2a)
exit 0
```

### evidence/wx-r4qa.txt
```
a-r4qa-Vm-v-ub.xml: ANSWER: VOUCHERS 2
  BP/26-27/0299 DATE 20260801 ALTERID 74 entries 6
  BP/26-27/0306 DATE 20260801 ALTERID 78 entries 6
exit 0
```

### evidence/wx-r4qb.txt
```
a-r4qb-Vm-v-ub.xml: ANSWER: VOUCHERS 2
  BP/26-27/0299 DATE 20260801 ALTERID 74 entries 6
  BP/26-27/0306 DATE 20260801 ALTERID 78 entries 6
exit 0
```

### evidence/same-r4q.txt
```
SAME: EQUAL (20966 B sha256 55a886f4f7c08ec7; 20966 B sha256 55a886f4f7c08ec7)
exit 0
```

### evidence/wx-r4wa.txt
```
a-r4wa-Vw-v-ub.xml: ANSWER: VOUCHERS 2
  BP/26-27/0299 DATE 20260801 ALTERID 74 entries 6
  BP/26-27/0306 DATE 20260801 ALTERID 78 entries 6
exit 0
```

### evidence/wx-r4wb.txt
```
a-r4wb-Vw-v-ub.xml: ANSWER: VOUCHERS 2
  BP/26-27/0299 DATE 20260801 ALTERID 74 entries 6
  BP/26-27/0306 DATE 20260801 ALTERID 78 entries 6
exit 0
```

### evidence/same-r4w.txt
```
SAME: EQUAL (186564 B sha256 5dc813068d5d30db; 186564 B sha256 5dc813068d5d30db)
exit 0
```

### evidence/judge2-r4c.txt
```
JUDGE2 ub census: OWN 2 OF 2, OTHERS 0
exit 0
```

### evidence/judge2-r4q.txt
```
JUDGE2 ub movement: OWN 2 OF 2, OTHERS 0
exit 0
```

### evidence/judge2-r4w.txt
```
JUDGE2 ub window: OWN 2 OF 2, OTHERS 0
exit 0
```

### evidence/cv-r5ca.txt
```
a-r5ca-C-v-ka.xml: ANSWER: VOUCHERS 3
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e ALTERID 70 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000037 ALTERID 79 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000038 ALTERID 80 DATE 20260801
exit 0
```

### evidence/cv-r5cb.txt
```
a-r5cb-C-v-ka.xml: ANSWER: VOUCHERS 3
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e ALTERID 70 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000037 ALTERID 79 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000038 ALTERID 80 DATE 20260801
exit 0
```

### evidence/same-r5c.txt
```
SAME: EQUAL (10262 B sha256 d6c79e2b1c51c296; 10262 B sha256 d6c79e2b1c51c296)
exit 0
```

### evidence/wx-r5qa.txt
```
a-r5qa-Vm-v-ka.xml: ANSWER: VOUCHERS 3
  BP/26-27/0292 DATE 20260801 ALTERID 70 entries 6
  BP/26-27/0307 DATE 20260801 ALTERID 79 entries 6
  BP/26-27/0308 DATE 20260801 ALTERID 80 entries 6
exit 0
```

### evidence/wx-r5qb.txt
```
a-r5qb-Vm-v-ka.xml: ANSWER: VOUCHERS 3
  BP/26-27/0292 DATE 20260801 ALTERID 70 entries 6
  BP/26-27/0307 DATE 20260801 ALTERID 79 entries 6
  BP/26-27/0308 DATE 20260801 ALTERID 80 entries 6
exit 0
```

### evidence/same-r5q.txt
```
SAME: EQUAL (29978 B sha256 9de9f2c6736552e9; 29978 B sha256 9de9f2c6736552e9)
exit 0
```

### evidence/wx-r5wa.txt
```
a-r5wa-Vw-v-ka.xml: ANSWER: VOUCHERS 3
  BP/26-27/0292 DATE 20260801 ALTERID 70 entries 6
  BP/26-27/0307 DATE 20260801 ALTERID 79 entries 6
  BP/26-27/0308 DATE 20260801 ALTERID 80 entries 6
exit 0
```

### evidence/wx-r5wb.txt
```
a-r5wb-Vw-v-ka.xml: ANSWER: VOUCHERS 3
  BP/26-27/0292 DATE 20260801 ALTERID 70 entries 6
  BP/26-27/0307 DATE 20260801 ALTERID 79 entries 6
  BP/26-27/0308 DATE 20260801 ALTERID 80 entries 6
exit 0
```

### evidence/same-r5w.txt
```
SAME: EQUAL (278376 B sha256 8ac6d771ca75938c; 278376 B sha256 8ac6d771ca75938c)
exit 0
```

### evidence/nm-r5la.txt
```
== a-r5la-L-lo-ka-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['2588']
  BILLREF 'BP/26-27/0292' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0307' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0308' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -3153.00
ANSWER: ROWS 3 (0 On Account)
exit 0
```

### evidence/nm-r5lb.txt
```
== a-r5lb-L-lo-ka-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['2588']
  BILLREF 'BP/26-27/0292' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0307' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0308' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -3153.00
ANSWER: ROWS 3 (0 On Account)
exit 0
```

### evidence/same-r5l.txt
```
SAME: EQUAL (2588 B sha256 bbbaaccf1c9ac677; 2588 B sha256 bbbaaccf1c9ac677)
exit 0
```

### evidence/judge2-r5c.txt
```
JUDGE2 ka census: OWN 3 OF 3, OTHERS 0
exit 0
```

### evidence/judge2-r5q.txt
```
JUDGE2 ka movement: OWN 3 OF 3, OTHERS 0
exit 0
```

### evidence/judge2-r5w.txt
```
JUDGE2 ka window: OWN 3 OF 3, OTHERS 0
exit 0
```

### evidence/judge2-r5l.txt
```
JUDGE2 ka outstandings: OWN 3 OF 3, OTHERS 0
exit 0
```

### evidence/cv-r6ca.txt
```
a-r6ca-C-v-kb.xml: ANSWER: VOUCHERS 3
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e ALTERID 70 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000037 ALTERID 79 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000038 ALTERID 80 DATE 20260801
exit 0
```

### evidence/cv-r6cb.txt
```
a-r6cb-C-v-kb.xml: ANSWER: VOUCHERS 3
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e ALTERID 70 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000037 ALTERID 79 DATE 20260801
  6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000038 ALTERID 80 DATE 20260801
exit 0
```

### evidence/same-r6c.txt
```
SAME: EQUAL (10262 B sha256 d6c79e2b1c51c296; 10262 B sha256 d6c79e2b1c51c296)
exit 0
```

### evidence/wx-r6qa.txt
```
a-r6qa-Vm-v-kb.xml: ANSWER: VOUCHERS 3
  BP/26-27/0292 DATE 20260801 ALTERID 70 entries 6
  BP/26-27/0307 DATE 20260801 ALTERID 79 entries 6
  BP/26-27/0308 DATE 20260801 ALTERID 80 entries 6
exit 0
```

### evidence/wx-r6qb.txt
```
a-r6qb-Vm-v-kb.xml: ANSWER: VOUCHERS 3
  BP/26-27/0292 DATE 20260801 ALTERID 70 entries 6
  BP/26-27/0307 DATE 20260801 ALTERID 79 entries 6
  BP/26-27/0308 DATE 20260801 ALTERID 80 entries 6
exit 0
```

### evidence/same-r6q.txt
```
SAME: EQUAL (29978 B sha256 9de9f2c6736552e9; 29978 B sha256 9de9f2c6736552e9)
exit 0
```

### evidence/wx-r6wa.txt
```
a-r6wa-Vw-v-kb.xml: ANSWER: VOUCHERS 3
  BP/26-27/0292 DATE 20260801 ALTERID 70 entries 6
  BP/26-27/0307 DATE 20260801 ALTERID 79 entries 6
  BP/26-27/0308 DATE 20260801 ALTERID 80 entries 6
exit 0
```

### evidence/wx-r6wb.txt
```
a-r6wb-Vw-v-kb.xml: ANSWER: VOUCHERS 3
  BP/26-27/0292 DATE 20260801 ALTERID 70 entries 6
  BP/26-27/0307 DATE 20260801 ALTERID 79 entries 6
  BP/26-27/0308 DATE 20260801 ALTERID 80 entries 6
exit 0
```

### evidence/same-r6w.txt
```
SAME: EQUAL (278376 B sha256 8ac6d771ca75938c; 278376 B sha256 8ac6d771ca75938c)
exit 0
```

### evidence/nm-r6la.txt
```
== a-r6la-L-lo-kb-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['2588']
  BILLREF 'BP/26-27/0292' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0307' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0308' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -3153.00
ANSWER: ROWS 3 (0 On Account)
exit 0
```

### evidence/nm-r6lb.txt
```
== a-r6lb-L-lo-kb-20260801.xml
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['2588']
  BILLREF 'BP/26-27/0292' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0307' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0308' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -3153.00
ANSWER: ROWS 3 (0 On Account)
exit 0
```

### evidence/same-r6l.txt
```
SAME: EQUAL (2588 B sha256 bbbaaccf1c9ac677; 2588 B sha256 bbbaaccf1c9ac677)
exit 0
```

### evidence/judge2-r6c.txt
```
JUDGE2 kb census: OWN 3 OF 3, OTHERS 0
exit 0
```

### evidence/judge2-r6q.txt
```
JUDGE2 kb movement: OWN 3 OF 3, OTHERS 0
exit 0
```

### evidence/judge2-r6w.txt
```
JUDGE2 kb window: OWN 3 OF 3, OTHERS 0
exit 0
```

### evidence/judge2-r6l.txt
```
JUDGE2 kb outstandings: OWN 3 OF 3, OTHERS 0
exit 0
```

### evidence/wx-e1.txt
```
a-e1-Vb-book.xml: ANSWER: VOUCHERS 23
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
  BP/26-27/0303 DATE 20260801 ALTERID 75 entries 6
  BP/26-27/0304 DATE 20260801 ALTERID 76 entries 6
  BP/26-27/0305 DATE 20260801 ALTERID 77 entries 6
  BP/26-27/0306 DATE 20260801 ALTERID 78 entries 6
  BP/26-27/0307 DATE 20260801 ALTERID 79 entries 6
  BP/26-27/0308 DATE 20260801 ALTERID 80 entries 6
exit 0
```

### evidence/same-e1.txt
```
SAME: EQUAL (1913112 B sha256 1a2b8dfcabdb22bc; 1913112 B sha256 1a2b8dfcabdb22bc)
exit 0
```

### evidence/count-e2.txt
```
== a-e2-M-marks.xml: 3406 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '80' ALTMSTID '252'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### evidence/same-e2.txt
```
== a-e2-M-marks.xml against a-k1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'80'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'80'", "'252'")
  equal
exit 0
```

### evidence/count-s9.txt
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
== a-s9-N-0303.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0304.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0305.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0306.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0307.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0308.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
exit 0
```

### evidence/nm-s9.txt
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

### evidence/cv-s9.txt
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

### evidence/names-s9.txt
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

### evidence/same-s9.txt
```
== a-s9-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
exit 0
```

### evidence/s9-against-s2.txt
```
EQUAL N-0281.xml
EQUAL N-0292.xml
EQUAL N-0293.xml
EQUAL N-0294.xml
EQUAL N-0295.xml
EQUAL N-0296.xml
EQUAL N-0297.xml
EQUAL N-0298.xml
EQUAL N-0299.xml
EQUAL N-0300.xml
EQUAL N-0301.xml
EQUAL B-bills.xml
EQUAL S-snap-20260401-20260801.xml
EQUAL C-book.xml
EQUAL G-cat.xml
EQUAL M-marks.xml
EQUAL N-0303.xml
EQUAL N-0304.xml
EQUAL N-0305.xml
EQUAL N-0306.xml
EQUAL N-0307.xml
EQUAL N-0308.xml
```

### evidence/bookcheck.txt
```
BRIDGE PILOT LAB: folder <N>, GUID hits 2, excluded GUID hits 0
folders named so: 1
exit 0
```

### evidence/pair-results.txt
```
2026-11-11T07:38:48Z pair k (ka, kb): ALTERED, AS SK RECORDED
2026-11-11T07:39:33Z pair q (qa, qb): BOTH
2026-11-11T07:40:20Z pair u (ua, ub): BOTH
```

### evidence/r8-results.txt
```
2026-11-11T07:50:14Z step 8 id qa (n=1): every send answered; empty answers under (a): 
2026-11-11T07:51:14Z step 8 id qb (n=2): every send answered; empty answers under (a): 
2026-11-11T07:52:04Z step 8 id ua (n=3): every send answered; empty answers under (a): 
2026-11-11T07:52:47Z step 8 id ub (n=4): every send answered; empty answers under (a): 
2026-11-11T07:53:44Z step 8 id ka (n=5): every send answered; empty answers under (a): 
2026-11-11T07:54:42Z step 8 id kb (n=6): every send answered; empty answers under (a): 
```

### evidence/tally-exe.txt
```
$ the program file of the Tally that answered (read after C10, with Tally left running at Select Company)
tally.exe file version 1.1.7.2; 18485952 B; sha256 856e5236a7664c0d8d580585e1c31b443720badd7a0078512d41f6c380b7ed73; last written 2026-10-25T05:34:45Z (equal to row 2480-SC's record)
```
