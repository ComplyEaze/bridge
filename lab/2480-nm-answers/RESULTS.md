# Row 2480-NM: five of the ten names were answered at step 3 as an alteration (CREATED 0, ALTERED 1), not a new ledger; the row stopped there, and no report read was sent

Run on 10 Nov 2026, 09:45:19Z to 09:58:48Z (15:15 to 15:28 IST), on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, `evidence/tally-exe.txt`; the Gateway shows no release number), by the sheet at `lab/2480-nm-sheet` (73cb01ae), on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic), loaded alone in a sitting of its own. Raw requests, one at a time, through `run.py` (curl, each answer's headers kept); no Bridge build. Every value below is M, measured by this run, in one run. Every read was answered, each within about 4.2 s (`evidence/gaps.txt`); no answer was empty. The window after each import was the Gateway, with no dialog (each capture byte-equal to `s1-gateway`).

## Result

**Not answered.** By the sheet's prompt rule, the row stopped at step 3, at the answer to `s3dt`. No step 4, 5 or 6 request was sent, so **no `Ledger Outstandings` read was sent for any name**, and none of the sheet's three questions is answered for any mark. The row's eleven report requests (`L-lo-<id>-20260801.xml`) were never sent.

**What step 3 measured instead.** Each answer below is whole in `evidence/count-<label>.txt`; each was one of 538 B (ledger) or 540 B (invoice).

| id | name | ledger import | its invoice | marks read before the ledger import (ALTVCHID, ALTMSTID) |
|---|---|---|---|---|
| `pl` | `BRIDGE Nm Plain` | `CREATED` 1, `ERRORS` 0 | `CREATED` 1, `LASTVCHID` 45 | 68, 245 |
| `hy` | `BRIDGE Nm Ax-Bx` | `CREATED` 1, `ERRORS` 0 | `CREATED` 1, `LASTVCHID` 46 | 69, 246 |
| `dt` | `BRIDGE Nm Ax.Bx` | **`CREATED` 0, `ALTERED` 1** | never sent | 70, 247 |
| `am` | `BRIDGE Nm Ax&Bx` | **`CREATED` 0, `ALTERED` 1** (sent after the stop) | never sent | 70, 248 |
| `sl` | `BRIDGE Nm Ax/Bx` | **`CREATED` 0, `ALTERED` 1** (sent after the stop) | never sent | 70, 249 |
| `lp` | `BRIDGE Nm Ax(Bx` | **`CREATED` 0, `ALTERED` 1** (sent after the stop) | never sent | 70, 250 |
| `rp` | `BRIDGE Nm Ax)Bx` | **`CREATED` 0, `ALTERED` 1** (sent after the stop) | never sent | 70, 251 |
| `ap` | `BRIDGE Nm Ax'Bx` | `CREATED` 1, `ERRORS` 0 (sent after the stop) | `CREATED` 1, `LASTVCHID` 47 | 70, 252 |
| `op` | `BRIDGE Nm Ax and Bx` | `CREATED` 1, `ERRORS` 0 (sent after the stop) | `CREATED` 1, `LASTVCHID` 48 | 71, 253 |
| `dd` | `BRIDGE Nm 2024-25` | `CREATED` 1, `ERRORS` 0 (sent after the stop) | `CREATED` 1, `LASTVCHID` 49 | 72, 254 |
| `sc` | `BRIDGE Nm नमूना` | `CREATED` 1, `ERRORS` 0 (sent after the stop) | `CREATED` 1, `LASTVCHID` 50 | 73, 255 |

- Every answer in the table had `ERRORS` 0, `EXCEPTIONS` 0, `DELETED` 0, `COMBINED` 0, `IGNORED` 0 and `CANCELLED` 0, and no LINEERROR.
- **The five `ALTERED` answers** (the dot, the ampersand, the slash, and each bracket alone) are none of the sheet's three import answers. Each raised ALTMSTID by one (247 before `s3dt`; 252 before `s3ap`), and none raised ALTVCHID. None of these answers names the master it altered.
- **What the book then held** (the state reads `c8s` and `c8b`, sent once each on the run lead's word after the stop):
  - the snapshot answered 32946 B with 24 `LEDGER` rows (s2: 18 rows). One row each for `pl`, `hy`, `ap`, `op`, `dd` and `sc`, each Sundry Debtors, bill-wise Yes, opening 0.00, closing -1051.00. **No row for `dt`, `am`, `sl`, `lp` or `rp`.**
  - **The ledger whose `NAME` attribute is `BRIDGE Nm Ax-Bx` (`hy`'s) had the name list `['BRIDGE Nm Ax)Bx']`, which is `rp`'s name,** the last of the five altered. The other five ledgers' name lists each equal their `NAME` attribute (`evidence/c8s-names.txt`, a read of the answer file only).
  - Bills Receivable answered 2808 B with 6 `BILLFIXED` groups: one each for `pl`, `hy`, `ap`, `op`, `dd` and `sc`, its own bill `BP/26-27/<its number>`, 1-Aug-26, -1051.00, due 1-Aug-26, overdue 0. None for the five; no group of any other party.
  - Marks: ALTVCHID 74, ALTMSTID 256.
- **Recorded, not judged.** The c8s read is consistent with each of the five imports matching `hy`'s ledger and rewriting its name list, the last one leaving `rp`'s name there. No read was taken between the five imports, so which master each of the first four altered is not measured. Nor is what Tally matched the names on. The `NAME` attribute of the row and of its bill group stayed `BRIDGE Nm Ax-Bx`.
- **By the sheet's Decides,** none of the listed outcomes was reached for any name. "The ledger's import refused (step 3)" does not describe the five: Tally did not refuse them, it altered an existing master. The hyphen, apostrophe, operator-word, digits and Devanagari names each became a ledger with its own bill, but no report read was sent for them either.

**The way back (step 7).** BK was restored by C6, with only the two expected prompts. Then s9 sent every file of s2, in s2's order:
- `--same-row` against s1: equal (ALTVCHID 68, ALTMSTID 245);
- each N read: 0 rows;
- Bills Receivable: 0 groups;
- the snapshot: 18 rows, no row for each of the eleven names;
- each of the 14 answers: sha256 equal to s2's (`evidence/s9-vs-s2.txt`: equal=14, differ=0).

So the book is as BK holds it, and the ledger with the altered name list is no longer in it.

**Deviations from the sheet and the lab's rules:**
1. **Eight write sends after the stop (step 3).** The row's `step3.sh` checked each import answer only for `CREATED` 1, `ERRORS` 0 and `EXCEPTIONS` 0. Anything else it treated as "a ledger's answer that is not that": recorded, its invoice not sent, and on to the next id. It did not test the sheet's list of the three import answers. So `s3dt`'s answer (`CREATED` 0, `ALTERED` 1), which by the sheet's "Any other answer ... : the prompt rule" stops the row, did not stop it.
   - The sends after that point, in order: `s3am`, `s3sl`, `s3lp` and `s3rp` (each `ALTERED` 1, its invoice not sent); then `s3ap` and `s4ap`, `s3op` and `s4op`, `s3dd` and `s4dd`, `s3sc` and `s4sc` (each `CREATED` 1). That is twelve imports in all (eight ledgers, four invoices); the run notes count eight write steps, the four ids with an invoice counted once each.
   - Each was sent after its own marks read and before-write pass. Each was followed by a window capture equal to the Gateway's (no dialog).
   - The last write was `s4sc` at 09:51:50Z. Nothing was sent after it until the run lead's word.
   - The operator saw it on reading the results at 09:52Z and reported to the run lead, who ordered (b) the state reads, then (a) the restore by C6 with the compare, then C10. Both were done as ordered.
   - `I-dt.xml`, `I-am.xml`, `I-sl.xml`, `I-lp.xml` and `I-rp.xml` were never sent. Every answer is kept whole (`answers/`).
2. **The state reads (C8).** On the run lead's word (b), `M-marks.xml`, `S-snap-20260401-20260801.xml` and `B-bills.xml` were sent once each, one at a time, as `c8m`, `c8s` and `c8b`. Before them, a window capture equal to the Gateway (`c8-before`) and `GET /status` 200 (51 B). These are PB's files byte for byte. Nothing else was sent.
3. **Bridge processes.** Three Bridge processes were running at the start. On the run lead's standing word none was ended, and they were left running at the close. No Bridge tool was called and no request went through Bridge.
4. **The start-up company.** Tally's start-up company loaded by itself, as the install is set to. It was shut by the K: Company label, Shut, Enter on it, before step 0, with no change to it.

**Beyond the sheet, recorded and not judged:**
- B-bills answered 46 B, an empty `ENVELOPE`, at s2 and s9: byte-equal to rows 2346-PB's and 2346-TD's s2 answers.
- The snapshot answered 25480 B at s2 and s9, with 18 `LEDGER` rows.
- The Gateway's company line carried "(e)" and "(Schedule Backup)" at step 1.
- The run lead's word given before the start, for this row: an `L-lo-` answer with HTTP 200 and `CONTENT-LENGTH: 0` is that ledger's result, never a stop. It was not needed: no `L-lo-` request was sent.

## Step 1: load, check and back up
- Sheet files: all 62 equal to `SHA256.txt` at 73cb01ae (sizes too).
- Before the start:
  - the book's data folder was found as the one whose `Company.1800` holds BRIDGE PILOT LAB and the book's GUID (one folder); it does not hold the excluded GUID (`evidence/bookcheck.txt`);
  - Tally was not running and nothing listened on 9001;
  - the backup folder was made new and empty.
- Tally was started once, at 09:45:19Z, from its desktop shortcut. Its start-up company was shut first (deviation 4).
  - At the first Enter of that shut, the capture tool refused the key while the desktop client had the keyboard focus. No key reached Tally. A title-bar click and a zoom showed the Shut list unchanged, and Enter was then pressed.
- C2 at 09:46:28Z, with Select Company showing: one `tally.exe`, which alone owns 9001; `GET /status` 200 (51 B); no company loaded.
- The book was loaded alone: `BRIDGE`, space, `PILOT`, space, `LAB` typed into Select Company's name filter, one row, BRIDGE PILOT LAB, 1-Apr-26 to 31-Mar-27 (`s1-select`), Enter, no prompt.
- The Gateway (`s1-gateway`): current period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026 (not before 1-Aug-2026); date of last entry 15-Sep-26. The PC's date: 10 Nov 2026 (IST).
- `s1` (09:47:00Z): one COMPANY row, the book's GUID, ALTVCHID 68, ALTMSTID 245. EXPECT met.
- BK by C5 (09:47Z to 09:48Z) to `C:\lab\2480-nm\backup-1` (Tally showed `C:\lab\2480-Nm\backup-1`, as the sheet says it may). The keys:
  - Data label, Backup & Restore, Backup; Backspace to the path;
  - Specify Path (Up 3 times); the path typed in two chunks; Enter, then Enter on the folder row;
  - `BRIDGE`, space, `PILOT`, space, `LAB` typed, the one row, Enter; End of List (Up 3 times), Enter;
  - "Accept ?" (`bk-accept`, 09:48:16Z) = y. No other message.

  One file, `TDBK1800_<company number>.001`, 90,051 B, sha256 `9dd64a22fa644bdac3d7a38013758afb7227342b9c9e0f172c92e64d53216a9a`. The Restore list showed it as 10-Nov-26 (3:18 pm).

## Step 2: the book before
- `s2` (09:48:46Z): each N read 0 rows; B-bills 0 groups, none of the eleven names; the snapshot 18 rows, "no row" for each of the eleven; M as s1. EXPECT met.

## Step 3: the set-up, and the stop
- Each write by the sheet's "Each write" (the row's `wr.sh`, driven by `step3.sh`):
  - `<label>m` and the before-write check against s1 (`evidence/bw-<label>.txt`), each passing;
  - the import alone, and the window captured at once (`<label>-after`): the Gateway, no dialog, each time;
  - `count20.py` on the answer (`evidence/count-<label>.txt`).
- `pl` (09:49:06Z to 09:49:27Z) and `hy` (09:50:04Z to 09:50:16Z): ledger and invoice each `CREATED` 1.
- `s3dt` (09:50:19Z): `CREATED` 0, `ALTERED` 1. The row stops here by the prompt rule. What followed is deviation 1.
- `s3am` to `s4sc` (09:50:26Z to 09:51:50Z): as in the Result.

## The state reads (C8), on the run lead's word
- `c8m`, `c8s`, `c8b` (09:53Z): as in the Result (`evidence/count-c8m.txt`, `evidence/nm-c8s.txt`, `evidence/nm-c8b.txt`, `evidence/c8s-names.txt`).

## Step 7: restore (C6) and compare
- Before the shut, `prep-c6.sh 7`:
  - the BK re-hashed equal to `evidence/bk.txt`, its folder holding only that file;
  - `restore-plan.txt` written;
  - the data folder listed (24 entries).

  The plan and listings stay on the lab PC.
- The restore, key by key:
  - K: Company label, Shut, Enter on BRIDGE PILOT LAB, no prompt (`c6-1-after-shut`, 09:54:49Z);
  - Data label, Backup & Restore, Restore. Restore Data Path offered `<Tally data folder>`; Specify Path (Up 3 times); the path typed in two chunks;
  - the list (`7-restore-list`) showed one row, BRIDGE PILOT LAB (`<company number>`), 10-Nov-26 (3:18 pm), as the plan. Enter on it; End of List, Enter;
  - "Accept ?" (`7-restore-accept`, 09:55:43Z) = y;
  - "Company data folder (`<company number>`) already exists." / "Selecting Skip will cancel the Restore process for this company." / "Selecting Replace will overwrite the existing folder.", buttons "R: Replace" and "S: Skip" (`7-restore-replace`, 09:55:56Z) = r. Select Company followed, no other message (`7-after-replace`).
- The data folder after the restore: after=17, before=24 entries.
  - Gone: `CmpSchMetaData.1800`, `Index.1800`, `TCMPSCHACCESS.TSF`, `TCMPSCHMESSAGE.TSF`, `TCMPSCHSTATE.TSF`, `TCMPSCHUPDATE.TSF`, `TINTMSG.TSF` (the same seven as row 2346-TD's restore on this book).
  - None new.
- Loaded alone: `BRIDGE`, space, `PILOT`, space, `LAB` typed, one row (`7-select`), Enter, no prompt. The Gateway (`7-gateway`, byte-equal to `s1-gateway`): period 1-Apr-26 to 31-Mar-27, current date Tuesday, 15-Sep-2026, date of last entry 15-Sep-26: as at step 1.
- `s9` (09:56:53Z): as in the Result (`evidence/same-s1-s9.txt`, `evidence/s9-vs-s2.txt`, `evidence/count-s9.txt`, `evidence/nm-s9.txt`).

## Step 8: close (C10)
- Shut by the K: Company label, Shut, BRIDGE PILOT LAB alone, Enter, no prompt: Select Company, no company loaded (`c10-no-company`, 09:57:37Z).
- C2 at 09:57:37Z, after the last request: one `tally.exe` (the same process as at step 0), which alone owns 9001; `GET /status` 200 (51 B). No XML request after the shut.
- Tally was then closed by its own Quit (Q: Quit clicked at Select Company), no prompt. At 09:58:48Z no `tally.exe` ran and nothing listened on 9001; one closed client connection to 9001 remained in `TIME_WAIT`. The three Bridge processes were left running (deviation 3).

## Files
- `answers/`: every answer and its headers file that may go, byte for byte (`kept-local.txt` lists those that stay on the lab PC, by size and sha256, with the reason).
- `evidence/`: the run notes, every `count20.py` and `nm.py` output, the before-write checks, the `wr.sh` consoles, the name lists of the c8s snapshot, the s9 compares, the gaps between sends, the release, and the answers' sizes and sha256. Process and port listings, the backup-folder listings and the data-folder listings stay on the lab PC. The BK record (`bk.txt`) goes with the company number replaced.
- `logs/`: `run.py`'s log of each send.
- `scripts/`: the row's helpers, with lab paths replaced. `step3.sh` is the script of deviation 1, as it ran.
- `SCREENSHOTS.txt`: every capture, by name, size and sha256. They stay on the lab PC.

## count20.py outputs, whole

### s1
```
== a-s1-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s2
```
== a-s2-N-0201.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0202.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0203.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0204.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0205.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0206.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0207.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0208.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0209.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0210.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0211.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s3plm
```
== a-s3plm-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s3pl
```
== a-s3pl-LM-pl.xml: 538 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s4plm
```
== a-s4plm-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s4pl
```
== a-s4pl-I-pl.xml: 540 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '45'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s3hym
```
== a-s3hym-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s3hy
```
== a-s3hy-LM-hy.xml: 538 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s4hym
```
== a-s4hym-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '247'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s4hy
```
== a-s4hy-I-hy.xml: 540 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '46'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s3dtm
```
== a-s3dtm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '247'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s3dt
```
== a-s3dt-LM-dt.xml: 538 bytes
  root RESPONSE
  CREATED '0'
  ALTERED '1'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s3amm
```
== a-s3amm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '248'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s3am
```
== a-s3am-LM-am.xml: 538 bytes
  root RESPONSE
  CREATED '0'
  ALTERED '1'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s3slm
```
== a-s3slm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '249'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s3sl
```
== a-s3sl-LM-sl.xml: 538 bytes
  root RESPONSE
  CREATED '0'
  ALTERED '1'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s3lpm
```
== a-s3lpm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '250'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s3lp
```
== a-s3lp-LM-lp.xml: 538 bytes
  root RESPONSE
  CREATED '0'
  ALTERED '1'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s3rpm
```
== a-s3rpm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '251'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s3rp
```
== a-s3rp-LM-rp.xml: 538 bytes
  root RESPONSE
  CREATED '0'
  ALTERED '1'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s3apm
```
== a-s3apm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '252'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s3ap
```
== a-s3ap-LM-ap.xml: 538 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s4apm
```
== a-s4apm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '253'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s4ap
```
== a-s4ap-I-ap.xml: 540 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '47'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s3opm
```
== a-s3opm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '71' ALTMSTID '253'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s3op
```
== a-s3op-LM-op.xml: 538 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s4opm
```
== a-s4opm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '71' ALTMSTID '254'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s4op
```
== a-s4op-I-op.xml: 540 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '48'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s3ddm
```
== a-s3ddm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '72' ALTMSTID '254'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s3dd
```
== a-s3dd-LM-dd.xml: 538 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s4ddm
```
== a-s4ddm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '72' ALTMSTID '255'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s4dd
```
== a-s4dd-I-dd.xml: 540 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '49'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s3scm
```
== a-s3scm-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '73' ALTMSTID '255'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s3sc
```
== a-s3sc-LM-sc.xml: 538 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s4scm
```
== a-s4scm-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '73' ALTMSTID '256'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s4sc
```
== a-s4sc-I-sc.xml: 540 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '50'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### c8m
```
== a-c8m-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '74' ALTMSTID '256'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s9
```
== a-s9-N-0201.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0202.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0203.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0204.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0205.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0206.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0207.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0208.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0209.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0210.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0211.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

## nm.py outputs, whole (the snapshot and Bills Receivable answers)

### s2
```
== a-s2-B-bills.xml
  BILLFIXED groups: 0
  pl 'BRIDGE Nm Plain': 0 groups
  hy 'BRIDGE Nm Ax-Bx': 0 groups
  dt 'BRIDGE Nm Ax.Bx': 0 groups
  am 'BRIDGE Nm Ax&Bx': 0 groups
  sl 'BRIDGE Nm Ax/Bx': 0 groups
  lp 'BRIDGE Nm Ax(Bx': 0 groups
  rp 'BRIDGE Nm Ax)Bx': 0 groups
  ap "BRIDGE Nm Ax'Bx": 0 groups
  op 'BRIDGE Nm Ax and Bx': 0 groups
  dd 'BRIDGE Nm 2024-25': 0 groups
  sc 'BRIDGE Nm नमूना': 0 groups
  groups of any other party: 0
== a-s2-S-snap-20260401-20260801.xml
a-s2-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 18
  pl 'BRIDGE Nm Plain': no row
  hy 'BRIDGE Nm Ax-Bx': no row
  dt 'BRIDGE Nm Ax.Bx': no row
  am 'BRIDGE Nm Ax&Bx': no row
  sl 'BRIDGE Nm Ax/Bx': no row
  lp 'BRIDGE Nm Ax(Bx': no row
  rp 'BRIDGE Nm Ax)Bx': no row
  ap "BRIDGE Nm Ax'Bx": no row
  op 'BRIDGE Nm Ax and Bx': no row
  dd 'BRIDGE Nm 2024-25': no row
  sc 'BRIDGE Nm नमूना': no row
exit 0
```

### c8s
```
== a-c8s-S-snap-20260401-20260801.xml
a-c8s-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 24
  pl 'BRIDGE Nm Plain': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  hy 'BRIDGE Nm Ax-Bx': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  dt 'BRIDGE Nm Ax.Bx': no row
  am 'BRIDGE Nm Ax&Bx': no row
  sl 'BRIDGE Nm Ax/Bx': no row
  lp 'BRIDGE Nm Ax(Bx': no row
  rp 'BRIDGE Nm Ax)Bx': no row
  ap "BRIDGE Nm Ax'Bx": PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  op 'BRIDGE Nm Ax and Bx': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  dd 'BRIDGE Nm 2024-25': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
  sc 'BRIDGE Nm नमूना': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
exit 0
```

### c8b
```
== a-c8b-B-bills.xml
  BILLFIXED groups: 6
  pl 'BRIDGE Nm Plain': 1 groups
    BILLREF 'BP/26-27/0201'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  hy 'BRIDGE Nm Ax-Bx': 1 groups
    BILLREF 'BP/26-27/0202'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  dt 'BRIDGE Nm Ax.Bx': 0 groups
  am 'BRIDGE Nm Ax&Bx': 0 groups
  sl 'BRIDGE Nm Ax/Bx': 0 groups
  lp 'BRIDGE Nm Ax(Bx': 0 groups
  rp 'BRIDGE Nm Ax)Bx': 0 groups
  ap "BRIDGE Nm Ax'Bx": 1 groups
    BILLREF 'BP/26-27/0208'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  op 'BRIDGE Nm Ax and Bx': 1 groups
    BILLREF 'BP/26-27/0209'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  dd 'BRIDGE Nm 2024-25': 1 groups
    BILLREF 'BP/26-27/0210'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  sc 'BRIDGE Nm नमूना': 1 groups
    BILLREF 'BP/26-27/0211'; BILLDATE '1-Aug-26'; BILLCL '-1051.00'; BILLDUE '1-Aug-26'; BILLOVERDUE '0'
  groups of any other party: 0
exit 0
```

### s9
```
== a-s9-B-bills.xml
  BILLFIXED groups: 0
  pl 'BRIDGE Nm Plain': 0 groups
  hy 'BRIDGE Nm Ax-Bx': 0 groups
  dt 'BRIDGE Nm Ax.Bx': 0 groups
  am 'BRIDGE Nm Ax&Bx': 0 groups
  sl 'BRIDGE Nm Ax/Bx': 0 groups
  lp 'BRIDGE Nm Ax(Bx': 0 groups
  rp 'BRIDGE Nm Ax)Bx': 0 groups
  ap "BRIDGE Nm Ax'Bx": 0 groups
  op 'BRIDGE Nm Ax and Bx': 0 groups
  dd 'BRIDGE Nm 2024-25': 0 groups
  sc 'BRIDGE Nm नमूना': 0 groups
  groups of any other party: 0
== a-s9-S-snap-20260401-20260801.xml
a-s9-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 18
  pl 'BRIDGE Nm Plain': no row
  hy 'BRIDGE Nm Ax-Bx': no row
  dt 'BRIDGE Nm Ax.Bx': no row
  am 'BRIDGE Nm Ax&Bx': no row
  sl 'BRIDGE Nm Ax/Bx': no row
  lp 'BRIDGE Nm Ax(Bx': no row
  rp 'BRIDGE Nm Ax)Bx': no row
  ap "BRIDGE Nm Ax'Bx": no row
  op 'BRIDGE Nm Ax and Bx': no row
  dd 'BRIDGE Nm 2024-25': no row
  sc 'BRIDGE Nm नमूना': no row
exit 0
```

## Write checks and consoles, whole

### s3plm: M against s1 (the before-write check)
```
== a-s3plm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4plm: M against s1 (the before-write check)
```
== a-s4plm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s3hym: M against s1 (the before-write check)
```
== a-s3hym-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s3hy: wr.sh console
```
M-marks.xml: 3396 bytes
== a-s3hym-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
== a-s3hym-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
LM-hy.xml: 538 bytes
printwindow True saved <LAB_DIR>/shots/s3hy-after.png
2026-11-10T09:50:07.830Z
== a-s3hy-LM-hy.xml: 538 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s4hym: M against s1 (the before-write check)
```
== a-s4hym-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'247'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4hy: wr.sh console
```
M-marks.xml: 3396 bytes
== a-s4hym-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '247'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
== a-s4hym-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'247'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
I-hy.xml: 540 bytes
printwindow True saved <LAB_DIR>/shots/s4hy-after.png
2026-11-10T09:50:15.272Z
== a-s4hy-I-hy.xml: 540 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '46'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s3dtm: M against s1 (the before-write check)
```
== a-s3dtm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'247'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s3dt: wr.sh console
```
M-marks.xml: 3396 bytes
== a-s3dtm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '247'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
== a-s3dtm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'247'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
LM-dt.xml: 538 bytes
printwindow True saved <LAB_DIR>/shots/s3dt-after.png
2026-11-10T09:50:22.545Z
== a-s3dt-LM-dt.xml: 538 bytes
  root RESPONSE
  CREATED '0'
  ALTERED '1'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s3amm: M against s1 (the before-write check)
```
== a-s3amm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s3am: wr.sh console
```
M-marks.xml: 3396 bytes
== a-s3amm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '248'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
== a-s3amm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'248'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
LM-am.xml: 538 bytes
printwindow True saved <LAB_DIR>/shots/s3am-after.png
2026-11-10T09:50:29.621Z
== a-s3am-LM-am.xml: 538 bytes
  root RESPONSE
  CREATED '0'
  ALTERED '1'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s3slm: M against s1 (the before-write check)
```
== a-s3slm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'249'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s3sl: wr.sh console
```
M-marks.xml: 3396 bytes
== a-s3slm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '249'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
== a-s3slm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'249'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
LM-sl.xml: 538 bytes
printwindow True saved <LAB_DIR>/shots/s3sl-after.png
2026-11-10T09:50:36.485Z
== a-s3sl-LM-sl.xml: 538 bytes
  root RESPONSE
  CREATED '0'
  ALTERED '1'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s3lpm: M against s1 (the before-write check)
```
== a-s3lpm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'250'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s3lp: wr.sh console
```
M-marks.xml: 3396 bytes
== a-s3lpm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '250'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
== a-s3lpm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'250'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
LM-lp.xml: 538 bytes
printwindow True saved <LAB_DIR>/shots/s3lp-after.png
2026-11-10T09:50:43.416Z
== a-s3lp-LM-lp.xml: 538 bytes
  root RESPONSE
  CREATED '0'
  ALTERED '1'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s3rpm: M against s1 (the before-write check)
```
== a-s3rpm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'251'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s3rp: wr.sh console
```
M-marks.xml: 3396 bytes
== a-s3rpm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '251'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
== a-s3rpm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'251'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
LM-rp.xml: 538 bytes
printwindow True saved <LAB_DIR>/shots/s3rp-after.png
2026-11-10T09:50:50.505Z
== a-s3rp-LM-rp.xml: 538 bytes
  root RESPONSE
  CREATED '0'
  ALTERED '1'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s3apm: M against s1 (the before-write check)
```
== a-s3apm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s3ap: wr.sh console
```
M-marks.xml: 3396 bytes
== a-s3apm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '252'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
== a-s3apm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'252'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
LM-ap.xml: 538 bytes
printwindow True saved <LAB_DIR>/shots/s3ap-after.png
2026-11-10T09:50:57.703Z
== a-s3ap-LM-ap.xml: 538 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s4apm: M against s1 (the before-write check)
```
== a-s4apm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'253'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4ap: wr.sh console
```
M-marks.xml: 3396 bytes
== a-s4apm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '253'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
== a-s4apm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'253'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
I-ap.xml: 540 bytes
printwindow True saved <LAB_DIR>/shots/s4ap-after.png
2026-11-10T09:51:05.222Z
== a-s4ap-I-ap.xml: 540 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '47'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s3opm: M against s1 (the before-write check)
```
== a-s3opm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'253'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s3op: wr.sh console
```
M-marks.xml: 3396 bytes
== a-s3opm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '71' ALTMSTID '253'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
== a-s3opm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'253'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
LM-op.xml: 538 bytes
printwindow True saved <LAB_DIR>/shots/s3op-after.png
2026-11-10T09:51:12.548Z
== a-s3op-LM-op.xml: 538 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s4opm: M against s1 (the before-write check)
```
== a-s4opm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'254'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4op: wr.sh console
```
M-marks.xml: 3396 bytes
== a-s4opm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '71' ALTMSTID '254'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
== a-s4opm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'254'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
I-op.xml: 540 bytes
printwindow True saved <LAB_DIR>/shots/s4op-after.png
2026-11-10T09:51:20.027Z
== a-s4op-I-op.xml: 540 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '48'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s3ddm: M against s1 (the before-write check)
```
== a-s3ddm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'254'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s3dd: wr.sh console
```
M-marks.xml: 3396 bytes
== a-s3ddm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '72' ALTMSTID '254'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
== a-s3ddm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'254'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
LM-dd.xml: 538 bytes
printwindow True saved <LAB_DIR>/shots/s3dd-after.png
2026-11-10T09:51:27.158Z
== a-s3dd-LM-dd.xml: 538 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s4ddm: M against s1 (the before-write check)
```
== a-s4ddm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'255'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4dd: wr.sh console
```
M-marks.xml: 3396 bytes
== a-s4ddm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '72' ALTMSTID '255'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
== a-s4ddm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'255'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
I-dd.xml: 540 bytes
printwindow True saved <LAB_DIR>/shots/s4dd-after.png
2026-11-10T09:51:34.309Z
== a-s4dd-I-dd.xml: 540 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '49'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s3scm: M against s1 (the before-write check)
```
== a-s3scm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'73'", "'255'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s3sc: wr.sh console
```
M-marks.xml: 3398 bytes
== a-s3scm-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '73' ALTMSTID '255'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
== a-s3scm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'73'", "'255'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
LM-sc.xml: 538 bytes
printwindow True saved <LAB_DIR>/shots/s3sc-after.png
2026-11-10T09:51:42.624Z
== a-s3sc-LM-sc.xml: 538 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### s4scm: M against s1 (the before-write check)
```
== a-s4scm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'73'", "'256'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4sc: wr.sh console
```
M-marks.xml: 3398 bytes
== a-s4scm-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '73' ALTMSTID '256'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
== a-s4scm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'73'", "'256'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
I-sc.xml: 540 bytes
printwindow True saved <LAB_DIR>/shots/s4sc-after.png
2026-11-10T09:51:49.943Z
== a-s4sc-I-sc.xml: 540 bytes
  root RESPONSE
  CREATED '1'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '50'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
exit 0
```

### c8s and s2: each snapshot row holding "BRIDGE Nm ", its NAME attribute and its name list (names-nm.py, a read of the answer files only)
```
== a-c8s-S-snap-20260401-20260801.xml
  NAME attribute 'BRIDGE Nm 2024-25'; name list ['BRIDGE Nm 2024-25']; the same
  NAME attribute "BRIDGE Nm Ax'Bx"; name list ["BRIDGE Nm Ax'Bx"]; the same
  NAME attribute 'BRIDGE Nm Ax and Bx'; name list ['BRIDGE Nm Ax and Bx']; the same
  NAME attribute 'BRIDGE Nm Ax-Bx'; name list ['BRIDGE Nm Ax)Bx']; DIFFERS
  NAME attribute 'BRIDGE Nm Plain'; name list ['BRIDGE Nm Plain']; the same
  NAME attribute 'BRIDGE Nm नमूना'; name list ['BRIDGE Nm नमूना']; the same
  rows with "BRIDGE Nm ": 6
== a-s2-S-snap-20260401-20260801.xml
  rows with "BRIDGE Nm ": 0
```

### s9: send.sh console
```
N-0201.xml: 3022 bytes
N-0202.xml: 3022 bytes
N-0203.xml: 3022 bytes
N-0204.xml: 3022 bytes
N-0205.xml: 3022 bytes
N-0206.xml: 3022 bytes
N-0207.xml: 3022 bytes
N-0208.xml: 3022 bytes
N-0209.xml: 3022 bytes
N-0210.xml: 3022 bytes
N-0211.xml: 3022 bytes
B-bills.xml: 46 bytes
S-snap-20260401-20260801.xml: 25480 bytes
M-marks.xml: 3394 bytes
== a-s9-B-bills.xml
  BILLFIXED groups: 0
  pl 'BRIDGE Nm Plain': 0 groups
  hy 'BRIDGE Nm Ax-Bx': 0 groups
  dt 'BRIDGE Nm Ax.Bx': 0 groups
  am 'BRIDGE Nm Ax&Bx': 0 groups
  sl 'BRIDGE Nm Ax/Bx': 0 groups
  lp 'BRIDGE Nm Ax(Bx': 0 groups
  rp 'BRIDGE Nm Ax)Bx': 0 groups
  ap "BRIDGE Nm Ax'Bx": 0 groups
  op 'BRIDGE Nm Ax and Bx': 0 groups
  dd 'BRIDGE Nm 2024-25': 0 groups
  sc 'BRIDGE Nm नमूना': 0 groups
  groups of any other party: 0
== a-s9-S-snap-20260401-20260801.xml
a-s9-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 18
  pl 'BRIDGE Nm Plain': no row
  hy 'BRIDGE Nm Ax-Bx': no row
  dt 'BRIDGE Nm Ax.Bx': no row
  am 'BRIDGE Nm Ax&Bx': no row
  sl 'BRIDGE Nm Ax/Bx': no row
  lp 'BRIDGE Nm Ax(Bx': no row
  rp 'BRIDGE Nm Ax)Bx': no row
  ap "BRIDGE Nm Ax'Bx": no row
  op 'BRIDGE Nm Ax and Bx': no row
  dd 'BRIDGE Nm 2024-25': no row
  sc 'BRIDGE Nm नमूना': no row
exit 0
== a-s9-N-0201.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0202.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0203.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0204.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0205.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0206.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0207.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0208.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0209.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0210.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0211.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s9: M against s1 (--same-row)
```
== a-s9-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
exit 0
```

### s9: every answer against s2 by sha256
```
$ sha256 and size of each s9 answer against s2's answer of the same file
N-0201.xml: s2 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; s9 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; EQUAL
N-0202.xml: s2 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; s9 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; EQUAL
N-0203.xml: s2 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; s9 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; EQUAL
N-0204.xml: s2 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; s9 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; EQUAL
N-0205.xml: s2 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; s9 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; EQUAL
N-0206.xml: s2 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; s9 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; EQUAL
N-0207.xml: s2 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; s9 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; EQUAL
N-0208.xml: s2 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; s9 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; EQUAL
N-0209.xml: s2 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; s9 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; EQUAL
N-0210.xml: s2 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; s9 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; EQUAL
N-0211.xml: s2 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; s9 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974; EQUAL
B-bills.xml: s2 46 B 8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3; s9 46 B 8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3; EQUAL
S-snap-20260401-20260801.xml: s2 25480 B ffe23742ab0793221f0136c5e119bde7d2e8fd2181f01e0f31520f61608eab69; s9 25480 B ffe23742ab0793221f0136c5e119bde7d2e8fd2181f01e0f31520f61608eab69; EQUAL
M-marks.xml: s2 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6; s9 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6; EQUAL
equal=14, differ=0
```

### step 1: the book's data folder and the excluded GUID
```
BRIDGE PILOT LAB: folder <N>, GUID hits 2, excluded GUID hits 0
folders named so: 1
```

### the program file of the Tally that answered
```
$ the program file of the Tally that answered (read after C10, with Tally closed)
tally.exe file version 1.1.7.2; 18485952 B; sha256 856e5236a7664c0d8d580585e1c31b443720badd7a0078512d41f6c380b7ed73; last written 2026-10-25T05:34:45Z
```

### each send's time to answer, and the gap before it
```
$ each send's time to answer and the gap from the previous send's answer to its start, seconds, from evidence/sends-utc.txt
s1: started 09:47:00.632Z
s1: answered in 0.6 s
s2: started 09:48:46.435Z; gap after s1 answered: 105.2 s
s2: answered in 2.0 s
s3plm: started 09:49:05.355Z; gap after s2 answered: 17.0 s
s3plm: answered in 0.4 s
s3pl: started 09:49:06.950Z; gap after s3plm answered: 1.2 s
s3pl: answered in 3.6 s
s4plm: started 09:49:22.182Z; gap after s3pl answered: 11.7 s
s4plm: answered in 0.4 s
s4pl: started 09:49:23.764Z; gap after s4plm answered: 1.2 s
s4pl: answered in 4.1 s
s3hym: started 09:50:02.520Z; gap after s4pl answered: 34.6 s
s3hym: answered in 0.5 s
s3hy: started 09:50:04.505Z; gap after s3hym answered: 1.5 s
s3hy: answered in 3.4 s
s4hym: started 09:50:09.614Z; gap after s3hy answered: 1.7 s
s4hym: answered in 0.5 s
s4hy: started 09:50:11.674Z; gap after s4hym answered: 1.5 s
s4hy: answered in 3.7 s
s3dtm: started 09:50:17.304Z; gap after s4hy answered: 1.9 s
s3dtm: answered in 0.5 s
s3dt: started 09:50:19.144Z; gap after s3dtm answered: 1.4 s
s3dt: answered in 3.5 s
s3amm: started 09:50:24.281Z; gap after s3dt answered: 1.6 s
s3amm: answered in 0.5 s
s3am: started 09:50:26.201Z; gap after s3amm answered: 1.5 s
s3am: answered in 3.5 s
s3slm: started 09:50:31.311Z; gap after s3am answered: 1.6 s
s3slm: answered in 0.5 s
s3sl: started 09:50:33.142Z; gap after s3slm answered: 1.4 s
s3sl: answered in 3.5 s
s3lpm: started 09:50:38.223Z; gap after s3sl answered: 1.6 s
s3lpm: answered in 0.5 s
s3lp: started 09:50:40.141Z; gap after s3lpm answered: 1.4 s
s3lp: answered in 3.4 s
s3rpm: started 09:50:45.195Z; gap after s3lp answered: 1.7 s
s3rpm: answered in 0.5 s
s3rp: started 09:50:47.090Z; gap after s3rpm answered: 1.4 s
s3rp: answered in 3.6 s
s3apm: started 09:50:52.342Z; gap after s3rp answered: 1.6 s
s3apm: answered in 0.5 s
s3ap: started 09:50:54.265Z; gap after s3apm answered: 1.4 s
s3ap: answered in 3.5 s
s4apm: started 09:50:59.421Z; gap after s3ap answered: 1.6 s
s4apm: answered in 0.5 s
s4ap: started 09:51:01.409Z; gap after s4apm answered: 1.5 s
s4ap: answered in 3.9 s
s3opm: started 09:51:07.168Z; gap after s4ap answered: 1.8 s
s3opm: answered in 0.5 s
s3op: started 09:51:09.131Z; gap after s3opm answered: 1.4 s
s3op: answered in 3.5 s
s4opm: started 09:51:14.264Z; gap after s3op answered: 1.6 s
s4opm: answered in 0.5 s
s4op: started 09:51:16.187Z; gap after s4opm answered: 1.4 s
s4op: answered in 3.9 s
s3ddm: started 09:51:21.989Z; gap after s4op answered: 1.9 s
s3ddm: answered in 0.5 s
s3dd: started 09:51:23.858Z; gap after s3ddm answered: 1.4 s
s3dd: answered in 3.4 s
s4ddm: started 09:51:28.928Z; gap after s3dd answered: 1.6 s
s4ddm: answered in 0.5 s
s4dd: started 09:51:30.733Z; gap after s4ddm answered: 1.3 s
s4dd: answered in 3.7 s
s3scm: started 09:51:36.230Z; gap after s4dd answered: 1.8 s
s3scm: answered in 0.5 s
s3sc: started 09:51:38.685Z; gap after s3scm answered: 1.9 s
s3sc: answered in 4.1 s
s4scm: started 09:51:44.391Z; gap after s3sc answered: 1.6 s
s4scm: answered in 0.5 s
s4sc: started 09:51:46.335Z; gap after s4scm answered: 1.4 s
s4sc: answered in 3.7 s
c8m: started 09:53:24.058Z; gap after s4sc answered: 94.0 s
c8m: answered in 0.5 s
c8s: started 09:53:30.397Z; gap after c8m answered: 5.9 s
c8s: answered in 0.4 s
c8b: started 09:53:37.597Z; gap after c8s answered: 6.8 s
c8b: answered in 0.4 s
s9: started 09:56:53.552Z; gap after c8b answered: 195.6 s
s9: answered in 2.0 s
```

### DESC/CMPINFO count lines of every answer
```
$ the DESC/CMPINFO count lines of every read answer that has them (LAB-COMMON C4: not rows); the names with a count other than 0, then the rest as "others 0"
a-s1-M-marks.xml: all 0 (names in all: 35)
a-s2-N-0201.xml: all 0 (names in all: 35)
a-s2-N-0202.xml: all 0 (names in all: 35)
a-s2-N-0203.xml: all 0 (names in all: 35)
a-s2-N-0204.xml: all 0 (names in all: 35)
a-s2-N-0205.xml: all 0 (names in all: 35)
a-s2-N-0206.xml: all 0 (names in all: 35)
a-s2-N-0207.xml: all 0 (names in all: 35)
a-s2-N-0208.xml: all 0 (names in all: 35)
a-s2-N-0209.xml: all 0 (names in all: 35)
a-s2-N-0210.xml: all 0 (names in all: 35)
a-s2-N-0211.xml: all 0 (names in all: 35)
a-s2-S-snap-20260401-20260801.xml: all 0 (names in all: 35)
a-s2-M-marks.xml: all 0 (names in all: 35)
a-s3plm-M-marks.xml: all 0 (names in all: 35)
a-s4plm-M-marks.xml: LEDGER 1; others 0 (names in all: 35)
a-s3hym-M-marks.xml: LEDGER 12; CURRENCY 1; TAXUNIT 1; VOUCHERNUMBERSERIES 1; VOUCHER 2; others 0 (names in all: 35)
a-s4hym-M-marks.xml: LEDGER 13; CURRENCY 1; TAXUNIT 1; VOUCHERNUMBERSERIES 1; VOUCHER 2; others 0 (names in all: 35)
a-s3dtm-M-marks.xml: LEDGER 24; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 4; others 0 (names in all: 35)
a-s3amm-M-marks.xml: LEDGER 25; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 4; others 0 (names in all: 35)
a-s3slm-M-marks.xml: LEDGER 26; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 4; others 0 (names in all: 35)
a-s3lpm-M-marks.xml: LEDGER 27; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 4; others 0 (names in all: 35)
a-s3rpm-M-marks.xml: LEDGER 28; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 4; others 0 (names in all: 35)
a-s3apm-M-marks.xml: LEDGER 29; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 4; others 0 (names in all: 35)
a-s4apm-M-marks.xml: LEDGER 30; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 4; others 0 (names in all: 35)
a-s3opm-M-marks.xml: LEDGER 41; CURRENCY 3; TAXUNIT 3; VOUCHERNUMBERSERIES 3; VOUCHER 6; others 0 (names in all: 35)
a-s4opm-M-marks.xml: LEDGER 42; CURRENCY 3; TAXUNIT 3; VOUCHERNUMBERSERIES 3; VOUCHER 6; others 0 (names in all: 35)
a-s3ddm-M-marks.xml: LEDGER 53; CURRENCY 4; TAXUNIT 4; VOUCHERNUMBERSERIES 4; VOUCHER 8; others 0 (names in all: 35)
a-s4ddm-M-marks.xml: LEDGER 54; CURRENCY 4; TAXUNIT 4; VOUCHERNUMBERSERIES 4; VOUCHER 8; others 0 (names in all: 35)
a-s3scm-M-marks.xml: LEDGER 65; CURRENCY 5; TAXUNIT 5; VOUCHERNUMBERSERIES 5; VOUCHER 10; others 0 (names in all: 35)
a-s4scm-M-marks.xml: LEDGER 66; CURRENCY 5; TAXUNIT 5; VOUCHERNUMBERSERIES 5; VOUCHER 10; others 0 (names in all: 35)
a-c8m-M-marks.xml: LEDGER 77; CURRENCY 6; TAXUNIT 6; VOUCHERNUMBERSERIES 6; VOUCHER 12; others 0 (names in all: 35)
a-c8s-S-snap-20260401-20260801.xml: LEDGER 77; CURRENCY 6; TAXUNIT 6; VOUCHERNUMBERSERIES 6; VOUCHER 12; others 0 (names in all: 35)
a-s9-N-0201.xml: all 0 (names in all: 35)
a-s9-N-0202.xml: all 0 (names in all: 35)
a-s9-N-0203.xml: all 0 (names in all: 35)
a-s9-N-0204.xml: all 0 (names in all: 35)
a-s9-N-0205.xml: all 0 (names in all: 35)
a-s9-N-0206.xml: all 0 (names in all: 35)
a-s9-N-0207.xml: all 0 (names in all: 35)
a-s9-N-0208.xml: all 0 (names in all: 35)
a-s9-N-0209.xml: all 0 (names in all: 35)
a-s9-N-0210.xml: all 0 (names in all: 35)
a-s9-N-0211.xml: all 0 (names in all: 35)
a-s9-S-snap-20260401-20260801.xml: all 0 (names in all: 35)
a-s9-M-marks.xml: all 0 (names in all: 35)
```

### sha256 of every answer and headers file
```
$ sha256sum and size of every answer and its headers file, in send order
a-s1-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s1-M-marks.xml.headers 134 B a8c6fe9f6545175fb5e08b9ee53ba543601c0436c80c231f7b009d73e5c8ccf0
a-s2-N-0201.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-N-0201.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s2-N-0202.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-N-0202.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s2-N-0203.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-N-0203.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s2-N-0204.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-N-0204.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s2-N-0205.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-N-0205.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s2-N-0206.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-N-0206.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s2-N-0207.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-N-0207.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s2-N-0208.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-N-0208.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s2-N-0209.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-N-0209.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s2-N-0210.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-N-0210.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s2-N-0211.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-N-0211.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s2-B-bills.xml 46 B 8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3
a-s2-B-bills.xml.headers 132 B 64cfab401cbb196abaed18a52645f02e91cc2cc3dd24eda9de98476a77e0f2da
a-s2-S-snap-20260401-20260801.xml 25480 B ffe23742ab0793221f0136c5e119bde7d2e8fd2181f01e0f31520f61608eab69
a-s2-S-snap-20260401-20260801.xml.headers 135 B 07ac9dc906b1544708fc63eca58a56aacb3bb969a35fde4f27a334add3ad1c64
a-s2-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s2-M-marks.xml.headers 134 B a8c6fe9f6545175fb5e08b9ee53ba543601c0436c80c231f7b009d73e5c8ccf0
a-s3plm-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s3plm-M-marks.xml.headers 134 B a8c6fe9f6545175fb5e08b9ee53ba543601c0436c80c231f7b009d73e5c8ccf0
a-s3pl-LM-pl.xml 538 B 1b72152a8fd20ccb9366277864bf32357b8891e536cde2a54992adaa60a97010
a-s3pl-LM-pl.xml.headers 133 B 19b28d1b18183bea529f520e0ce9251ee2655e6b58302a8b47562695a42a91b7
a-s4plm-M-marks.xml 3394 B 49bdfb3f21e1db5f7d0c4991efa5683bbb0d9cb180ad8092b961024713ac66a7
a-s4plm-M-marks.xml.headers 134 B a8c6fe9f6545175fb5e08b9ee53ba543601c0436c80c231f7b009d73e5c8ccf0
a-s4pl-I-pl.xml 540 B fdcd8435c6fcf95ab624cbe2cff2c887078df500626f7c9c37cfa834a1cdb6ea
a-s4pl-I-pl.xml.headers 133 B 2a0eba722d654c2b2fa59d71608c67e3849d25c56207357fa06b0239977a4670
a-s3hym-M-marks.xml 3396 B 17c5bde53bf5e459f4ae36fce8ee62e8bfe5eb636f47f1970faff44c230a59ed
a-s3hym-M-marks.xml.headers 134 B db9b137db28569a10d72e1f9edb1f97fa4dc3c0f0c092ea2d8b3e1ba4cae016c
a-s3hy-LM-hy.xml 538 B 1b72152a8fd20ccb9366277864bf32357b8891e536cde2a54992adaa60a97010
a-s3hy-LM-hy.xml.headers 133 B 19b28d1b18183bea529f520e0ce9251ee2655e6b58302a8b47562695a42a91b7
a-s4hym-M-marks.xml 3396 B 47797a8a843b6ca828a17c3fa9323218fb8f7f4f24874878d20a77811616381f
a-s4hym-M-marks.xml.headers 134 B db9b137db28569a10d72e1f9edb1f97fa4dc3c0f0c092ea2d8b3e1ba4cae016c
a-s4hy-I-hy.xml 540 B 32e7a29b201c773547aa47bc95bd3eda8af83aeb6e35090303b9b27bcd34eec5
a-s4hy-I-hy.xml.headers 133 B 2a0eba722d654c2b2fa59d71608c67e3849d25c56207357fa06b0239977a4670
a-s3dtm-M-marks.xml 3396 B 83a554d4a5e4cd3a8ee41fa0950dd681b88d7240505bdf442755013b337e7b07
a-s3dtm-M-marks.xml.headers 134 B db9b137db28569a10d72e1f9edb1f97fa4dc3c0f0c092ea2d8b3e1ba4cae016c
a-s3dt-LM-dt.xml 538 B a373b08c8c891b2608c663ed2b11c83979e6908299f9c1088426c246412b1901
a-s3dt-LM-dt.xml.headers 133 B 19b28d1b18183bea529f520e0ce9251ee2655e6b58302a8b47562695a42a91b7
a-s3amm-M-marks.xml 3396 B 8067cfcf655f328c6a593c00390ed032552d633b43ce71de6f70d65cd8bafd23
a-s3amm-M-marks.xml.headers 134 B db9b137db28569a10d72e1f9edb1f97fa4dc3c0f0c092ea2d8b3e1ba4cae016c
a-s3am-LM-am.xml 538 B a373b08c8c891b2608c663ed2b11c83979e6908299f9c1088426c246412b1901
a-s3am-LM-am.xml.headers 133 B 19b28d1b18183bea529f520e0ce9251ee2655e6b58302a8b47562695a42a91b7
a-s3slm-M-marks.xml 3396 B a3bf51b7b944931a23bffd1bab0bc2e0a6625829ecfd3269366dec3e8d39d68b
a-s3slm-M-marks.xml.headers 134 B db9b137db28569a10d72e1f9edb1f97fa4dc3c0f0c092ea2d8b3e1ba4cae016c
a-s3sl-LM-sl.xml 538 B a373b08c8c891b2608c663ed2b11c83979e6908299f9c1088426c246412b1901
a-s3sl-LM-sl.xml.headers 133 B 19b28d1b18183bea529f520e0ce9251ee2655e6b58302a8b47562695a42a91b7
a-s3lpm-M-marks.xml 3396 B 6d1885ece32e6c14d72fc9e908a2384fb58fcbaba3b0b6b7ca2a5a81389f0f05
a-s3lpm-M-marks.xml.headers 134 B db9b137db28569a10d72e1f9edb1f97fa4dc3c0f0c092ea2d8b3e1ba4cae016c
a-s3lp-LM-lp.xml 538 B a373b08c8c891b2608c663ed2b11c83979e6908299f9c1088426c246412b1901
a-s3lp-LM-lp.xml.headers 133 B 19b28d1b18183bea529f520e0ce9251ee2655e6b58302a8b47562695a42a91b7
a-s3rpm-M-marks.xml 3396 B 88e49df078b19117f9657dba7895f72cda816eff3830a94b504f02cfa6ce8d8c
a-s3rpm-M-marks.xml.headers 134 B db9b137db28569a10d72e1f9edb1f97fa4dc3c0f0c092ea2d8b3e1ba4cae016c
a-s3rp-LM-rp.xml 538 B a373b08c8c891b2608c663ed2b11c83979e6908299f9c1088426c246412b1901
a-s3rp-LM-rp.xml.headers 133 B 19b28d1b18183bea529f520e0ce9251ee2655e6b58302a8b47562695a42a91b7
a-s3apm-M-marks.xml 3396 B 5266ef567fb81e5072f9eeb1fb7aa023bf4da1fe71e37ed914faa8272f68dc81
a-s3apm-M-marks.xml.headers 134 B db9b137db28569a10d72e1f9edb1f97fa4dc3c0f0c092ea2d8b3e1ba4cae016c
a-s3ap-LM-ap.xml 538 B 1b72152a8fd20ccb9366277864bf32357b8891e536cde2a54992adaa60a97010
a-s3ap-LM-ap.xml.headers 133 B 19b28d1b18183bea529f520e0ce9251ee2655e6b58302a8b47562695a42a91b7
a-s4apm-M-marks.xml 3396 B fa3b91db7748575e3c41bf587c4790488a6df46df126c9d7b7779b5d6bdae58d
a-s4apm-M-marks.xml.headers 134 B db9b137db28569a10d72e1f9edb1f97fa4dc3c0f0c092ea2d8b3e1ba4cae016c
a-s4ap-I-ap.xml 540 B f8384a874e1d8a6be92c2969f94e2887fb700768f6e111b6b3f649cd13ffdc7e
a-s4ap-I-ap.xml.headers 133 B 2a0eba722d654c2b2fa59d71608c67e3849d25c56207357fa06b0239977a4670
a-s3opm-M-marks.xml 3396 B f0db456f9443a4f014ddb132070b2470c0f857a647c08f378092cd6ce18a9df4
a-s3opm-M-marks.xml.headers 134 B db9b137db28569a10d72e1f9edb1f97fa4dc3c0f0c092ea2d8b3e1ba4cae016c
a-s3op-LM-op.xml 538 B 1b72152a8fd20ccb9366277864bf32357b8891e536cde2a54992adaa60a97010
a-s3op-LM-op.xml.headers 133 B 19b28d1b18183bea529f520e0ce9251ee2655e6b58302a8b47562695a42a91b7
a-s4opm-M-marks.xml 3396 B d2e4a1f035852b44c066534f0109a35d3d2a0e0da7866e9d977226eb1e150318
a-s4opm-M-marks.xml.headers 134 B db9b137db28569a10d72e1f9edb1f97fa4dc3c0f0c092ea2d8b3e1ba4cae016c
a-s4op-I-op.xml 540 B e6540f6534095f5608f3c59f802c1f40a3ae13529a7e9041847b460f03c32fec
a-s4op-I-op.xml.headers 133 B 2a0eba722d654c2b2fa59d71608c67e3849d25c56207357fa06b0239977a4670
a-s3ddm-M-marks.xml 3396 B b68f3588733365bab7a12e380ee8a20c9725c0271691673f47cd08b740221142
a-s3ddm-M-marks.xml.headers 134 B db9b137db28569a10d72e1f9edb1f97fa4dc3c0f0c092ea2d8b3e1ba4cae016c
a-s3dd-LM-dd.xml 538 B 1b72152a8fd20ccb9366277864bf32357b8891e536cde2a54992adaa60a97010
a-s3dd-LM-dd.xml.headers 133 B 19b28d1b18183bea529f520e0ce9251ee2655e6b58302a8b47562695a42a91b7
a-s4ddm-M-marks.xml 3396 B e545412464330d13fc93ac7215405be2b1358ae418d82a22868d73e6ab4ed0cc
a-s4ddm-M-marks.xml.headers 134 B db9b137db28569a10d72e1f9edb1f97fa4dc3c0f0c092ea2d8b3e1ba4cae016c
a-s4dd-I-dd.xml 540 B 93e331323bb1235565ab31f2ddbbaf7afaa752ce6770d0ae7ae145d7b3244234
a-s4dd-I-dd.xml.headers 133 B 2a0eba722d654c2b2fa59d71608c67e3849d25c56207357fa06b0239977a4670
a-s3scm-M-marks.xml 3398 B b63d95cc40674594c3d220bd8d12682e48c18aad9fb1f7ea7f0f47622a6c4e1e
a-s3scm-M-marks.xml.headers 134 B 4d2c776864af2995685156531c0e1bd6dfa88116418863514e54213f2a5ee6a0
a-s3sc-LM-sc.xml 538 B 1b72152a8fd20ccb9366277864bf32357b8891e536cde2a54992adaa60a97010
a-s3sc-LM-sc.xml.headers 133 B 19b28d1b18183bea529f520e0ce9251ee2655e6b58302a8b47562695a42a91b7
a-s4scm-M-marks.xml 3398 B e7ec3c6e89a2133e0e471f8827ee1c908794408591262d1c5c589431895983b0
a-s4scm-M-marks.xml.headers 134 B 4d2c776864af2995685156531c0e1bd6dfa88116418863514e54213f2a5ee6a0
a-s4sc-I-sc.xml 540 B 200c17e120c3ff582d28c9fce94b127ed22aaa4e5d10bbdd1d18e97a767f40c3
a-s4sc-I-sc.xml.headers 133 B 2a0eba722d654c2b2fa59d71608c67e3849d25c56207357fa06b0239977a4670
a-c8m-M-marks.xml 3398 B 35e22b0a05113b5104198c748755e5681187a9f33bbe3a0c17e3eb3bc84b30d7
a-c8m-M-marks.xml.headers 134 B 4d2c776864af2995685156531c0e1bd6dfa88116418863514e54213f2a5ee6a0
a-c8s-S-snap-20260401-20260801.xml 32946 B 180403aa3b959c0bc5fd2478bba6bbd983a105f3a27d62c874088f73522adb6e
a-c8s-S-snap-20260401-20260801.xml.headers 135 B f156288232582fbeaec1a2e35e0b59ce2320b6b259bef0f76a2b320bcbeaf5e4
a-c8b-B-bills.xml 2808 B fdef0529ded52d9ab45c1ed4403c04a4d6f543ca607633f11eadd2d468e10ea2
a-c8b-B-bills.xml.headers 134 B 86b41d539535c2396e4eec3dccd18722848632ac0ff7b8582592bc0fadd36e3e
a-s9-N-0201.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s9-N-0201.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s9-N-0202.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s9-N-0202.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s9-N-0203.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s9-N-0203.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s9-N-0204.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s9-N-0204.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s9-N-0205.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s9-N-0205.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s9-N-0206.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s9-N-0206.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s9-N-0207.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s9-N-0207.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s9-N-0208.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s9-N-0208.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s9-N-0209.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s9-N-0209.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s9-N-0210.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s9-N-0210.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s9-N-0211.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s9-N-0211.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s9-B-bills.xml 46 B 8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3
a-s9-B-bills.xml.headers 132 B 64cfab401cbb196abaed18a52645f02e91cc2cc3dd24eda9de98476a77e0f2da
a-s9-S-snap-20260401-20260801.xml 25480 B ffe23742ab0793221f0136c5e119bde7d2e8fd2181f01e0f31520f61608eab69
a-s9-S-snap-20260401-20260801.xml.headers 135 B 07ac9dc906b1544708fc63eca58a56aacb3bb969a35fde4f27a334add3ad1c64
a-s9-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s9-M-marks.xml.headers 134 B a8c6fe9f6545175fb5e08b9ee53ba543601c0436c80c231f7b009d73e5c8ccf0
```

### each send's start and answer, UTC
```
2026-11-10T09:47:00.632Z s1 M-marks.xml (hashes equal to the list)
2026-11-10T09:47:01.196Z s1 answered
2026-11-10T09:48:46.435Z s2 N-0201.xml N-0202.xml N-0203.xml N-0204.xml N-0205.xml N-0206.xml N-0207.xml N-0208.xml N-0209.xml N-0210.xml N-0211.xml B-bills.xml S-snap-20260401-20260801.xml M-marks.xml (hashes equal to the list)
2026-11-10T09:48:48.388Z s2 answered
2026-11-10T09:49:05.355Z s3plm M-marks.xml (hashes equal to the list)
2026-11-10T09:49:05.749Z s3plm answered
2026-11-10T09:49:06.950Z s3pl LM-pl.xml (hashes equal to the list)
2026-11-10T09:49:10.530Z s3pl answered
2026-11-10T09:49:22.182Z s4plm M-marks.xml (hashes equal to the list)
2026-11-10T09:49:22.566Z s4plm answered
2026-11-10T09:49:23.764Z s4pl I-pl.xml (hashes equal to the list)
2026-11-10T09:49:27.906Z s4pl answered
2026-11-10T09:50:02.520Z s3hym M-marks.xml (hashes equal to the list)
2026-11-10T09:50:03.038Z s3hym answered
2026-11-10T09:50:04.505Z s3hy LM-hy.xml (hashes equal to the list)
2026-11-10T09:50:07.932Z s3hy answered
2026-11-10T09:50:09.614Z s4hym M-marks.xml (hashes equal to the list)
2026-11-10T09:50:10.127Z s4hym answered
2026-11-10T09:50:11.674Z s4hy I-hy.xml (hashes equal to the list)
2026-11-10T09:50:15.386Z s4hy answered
2026-11-10T09:50:17.304Z s3dtm M-marks.xml (hashes equal to the list)
2026-11-10T09:50:17.760Z s3dtm answered
2026-11-10T09:50:19.144Z s3dt LM-dt.xml (hashes equal to the list)
2026-11-10T09:50:22.640Z s3dt answered
2026-11-10T09:50:24.281Z s3amm M-marks.xml (hashes equal to the list)
2026-11-10T09:50:24.745Z s3amm answered
2026-11-10T09:50:26.201Z s3am LM-am.xml (hashes equal to the list)
2026-11-10T09:50:29.724Z s3am answered
2026-11-10T09:50:31.311Z s3slm M-marks.xml (hashes equal to the list)
2026-11-10T09:50:31.782Z s3slm answered
2026-11-10T09:50:33.142Z s3sl LM-sl.xml (hashes equal to the list)
2026-11-10T09:50:36.592Z s3sl answered
2026-11-10T09:50:38.223Z s3lpm M-marks.xml (hashes equal to the list)
2026-11-10T09:50:38.712Z s3lpm answered
2026-11-10T09:50:40.141Z s3lp LM-lp.xml (hashes equal to the list)
2026-11-10T09:50:43.517Z s3lp answered
2026-11-10T09:50:45.195Z s3rpm M-marks.xml (hashes equal to the list)
2026-11-10T09:50:45.655Z s3rpm answered
2026-11-10T09:50:47.090Z s3rp LM-rp.xml (hashes equal to the list)
2026-11-10T09:50:50.717Z s3rp answered
2026-11-10T09:50:52.342Z s3apm M-marks.xml (hashes equal to the list)
2026-11-10T09:50:52.824Z s3apm answered
2026-11-10T09:50:54.265Z s3ap LM-ap.xml (hashes equal to the list)
2026-11-10T09:50:57.773Z s3ap answered
2026-11-10T09:50:59.421Z s4apm M-marks.xml (hashes equal to the list)
2026-11-10T09:50:59.946Z s4apm answered
2026-11-10T09:51:01.409Z s4ap I-ap.xml (hashes equal to the list)
2026-11-10T09:51:05.334Z s4ap answered
2026-11-10T09:51:07.168Z s3opm M-marks.xml (hashes equal to the list)
2026-11-10T09:51:07.684Z s3opm answered
2026-11-10T09:51:09.131Z s3op LM-op.xml (hashes equal to the list)
2026-11-10T09:51:12.637Z s3op answered
2026-11-10T09:51:14.264Z s4opm M-marks.xml (hashes equal to the list)
2026-11-10T09:51:14.740Z s4opm answered
2026-11-10T09:51:16.187Z s4op I-op.xml (hashes equal to the list)
2026-11-10T09:51:20.108Z s4op answered
2026-11-10T09:51:21.989Z s3ddm M-marks.xml (hashes equal to the list)
2026-11-10T09:51:22.460Z s3ddm answered
2026-11-10T09:51:23.858Z s3dd LM-dd.xml (hashes equal to the list)
2026-11-10T09:51:27.279Z s3dd answered
2026-11-10T09:51:28.928Z s4ddm M-marks.xml (hashes equal to the list)
2026-11-10T09:51:29.384Z s4ddm answered
2026-11-10T09:51:30.733Z s4dd I-dd.xml (hashes equal to the list)
2026-11-10T09:51:34.397Z s4dd answered
2026-11-10T09:51:36.230Z s3scm M-marks.xml (hashes equal to the list)
2026-11-10T09:51:36.766Z s3scm answered
2026-11-10T09:51:38.685Z s3sc LM-sc.xml (hashes equal to the list)
2026-11-10T09:51:42.744Z s3sc answered
2026-11-10T09:51:44.391Z s4scm M-marks.xml (hashes equal to the list)
2026-11-10T09:51:44.912Z s4scm answered
2026-11-10T09:51:46.335Z s4sc I-sc.xml (hashes equal to the list)
2026-11-10T09:51:50.043Z s4sc answered
2026-11-10T09:53:24.058Z c8m M-marks.xml (hashes equal to the list)
2026-11-10T09:53:24.543Z c8m answered
2026-11-10T09:53:30.397Z c8s S-snap-20260401-20260801.xml (hashes equal to the list)
2026-11-10T09:53:30.757Z c8s answered
2026-11-10T09:53:37.597Z c8b B-bills.xml (hashes equal to the list)
2026-11-10T09:53:37.991Z c8b answered
2026-11-10T09:56:53.552Z s9 N-0201.xml N-0202.xml N-0203.xml N-0204.xml N-0205.xml N-0206.xml N-0207.xml N-0208.xml N-0209.xml N-0210.xml N-0211.xml B-bills.xml S-snap-20260401-20260801.xml M-marks.xml (hashes equal to the list)
2026-11-10T09:56:55.504Z s9 answered
```

## run.py logs (each send: local IST time to the second, request, answer file, curl exit, bytes)

### s1
```
15:17:01	M-marks.xml	a-s1-M-marks.xml	curl 0	3394 bytes
```

### s2
```
15:18:46	N-0201.xml	a-s2-N-0201.xml	curl 0	3022 bytes
15:18:46	N-0202.xml	a-s2-N-0202.xml	curl 0	3022 bytes
15:18:47	N-0203.xml	a-s2-N-0203.xml	curl 0	3022 bytes
15:18:47	N-0204.xml	a-s2-N-0204.xml	curl 0	3022 bytes
15:18:47	N-0205.xml	a-s2-N-0205.xml	curl 0	3022 bytes
15:18:47	N-0206.xml	a-s2-N-0206.xml	curl 0	3022 bytes
15:18:47	N-0207.xml	a-s2-N-0207.xml	curl 0	3022 bytes
15:18:47	N-0208.xml	a-s2-N-0208.xml	curl 0	3022 bytes
15:18:47	N-0209.xml	a-s2-N-0209.xml	curl 0	3022 bytes
15:18:47	N-0210.xml	a-s2-N-0210.xml	curl 0	3022 bytes
15:18:47	N-0211.xml	a-s2-N-0211.xml	curl 0	3022 bytes
15:18:48	B-bills.xml	a-s2-B-bills.xml	curl 0	46 bytes
15:18:48	S-snap-20260401-20260801.xml	a-s2-S-snap-20260401-20260801.xml	curl 0	25480 bytes
15:18:48	M-marks.xml	a-s2-M-marks.xml	curl 0	3394 bytes
```

### s3plm
```
15:19:05	M-marks.xml	a-s3plm-M-marks.xml	curl 0	3394 bytes
```

### s3pl
```
15:19:07	LM-pl.xml	a-s3pl-LM-pl.xml	curl 0	538 bytes
```

### s4plm
```
15:19:22	M-marks.xml	a-s4plm-M-marks.xml	curl 0	3394 bytes
```

### s4pl
```
15:19:24	I-pl.xml	a-s4pl-I-pl.xml	curl 0	540 bytes
```

### s3hym
```
15:20:02	M-marks.xml	a-s3hym-M-marks.xml	curl 0	3396 bytes
```

### s3hy
```
15:20:04	LM-hy.xml	a-s3hy-LM-hy.xml	curl 0	538 bytes
```

### s4hym
```
15:20:09	M-marks.xml	a-s4hym-M-marks.xml	curl 0	3396 bytes
```

### s4hy
```
15:20:12	I-hy.xml	a-s4hy-I-hy.xml	curl 0	540 bytes
```

### s3dtm
```
15:20:17	M-marks.xml	a-s3dtm-M-marks.xml	curl 0	3396 bytes
```

### s3dt
```
15:20:19	LM-dt.xml	a-s3dt-LM-dt.xml	curl 0	538 bytes
```

### s3amm
```
15:20:24	M-marks.xml	a-s3amm-M-marks.xml	curl 0	3396 bytes
```

### s3am
```
15:20:26	LM-am.xml	a-s3am-LM-am.xml	curl 0	538 bytes
```

### s3slm
```
15:20:31	M-marks.xml	a-s3slm-M-marks.xml	curl 0	3396 bytes
```

### s3sl
```
15:20:33	LM-sl.xml	a-s3sl-LM-sl.xml	curl 0	538 bytes
```

### s3lpm
```
15:20:38	M-marks.xml	a-s3lpm-M-marks.xml	curl 0	3396 bytes
```

### s3lp
```
15:20:40	LM-lp.xml	a-s3lp-LM-lp.xml	curl 0	538 bytes
```

### s3rpm
```
15:20:45	M-marks.xml	a-s3rpm-M-marks.xml	curl 0	3396 bytes
```

### s3rp
```
15:20:47	LM-rp.xml	a-s3rp-LM-rp.xml	curl 0	538 bytes
```

### s3apm
```
15:20:52	M-marks.xml	a-s3apm-M-marks.xml	curl 0	3396 bytes
```

### s3ap
```
15:20:54	LM-ap.xml	a-s3ap-LM-ap.xml	curl 0	538 bytes
```

### s4apm
```
15:20:59	M-marks.xml	a-s4apm-M-marks.xml	curl 0	3396 bytes
```

### s4ap
```
15:21:01	I-ap.xml	a-s4ap-I-ap.xml	curl 0	540 bytes
```

### s3opm
```
15:21:07	M-marks.xml	a-s3opm-M-marks.xml	curl 0	3396 bytes
```

### s3op
```
15:21:09	LM-op.xml	a-s3op-LM-op.xml	curl 0	538 bytes
```

### s4opm
```
15:21:14	M-marks.xml	a-s4opm-M-marks.xml	curl 0	3396 bytes
```

### s4op
```
15:21:16	I-op.xml	a-s4op-I-op.xml	curl 0	540 bytes
```

### s3ddm
```
15:21:22	M-marks.xml	a-s3ddm-M-marks.xml	curl 0	3396 bytes
```

### s3dd
```
15:21:24	LM-dd.xml	a-s3dd-LM-dd.xml	curl 0	538 bytes
```

### s4ddm
```
15:21:29	M-marks.xml	a-s4ddm-M-marks.xml	curl 0	3396 bytes
```

### s4dd
```
15:21:31	I-dd.xml	a-s4dd-I-dd.xml	curl 0	540 bytes
```

### s3scm
```
15:21:36	M-marks.xml	a-s3scm-M-marks.xml	curl 0	3398 bytes
```

### s3sc
```
15:21:39	LM-sc.xml	a-s3sc-LM-sc.xml	curl 0	538 bytes
```

### s4scm
```
15:21:44	M-marks.xml	a-s4scm-M-marks.xml	curl 0	3398 bytes
```

### s4sc
```
15:21:46	I-sc.xml	a-s4sc-I-sc.xml	curl 0	540 bytes
```

### c8m
```
15:23:24	M-marks.xml	a-c8m-M-marks.xml	curl 0	3398 bytes
```

### c8s
```
15:23:30	S-snap-20260401-20260801.xml	a-c8s-S-snap-20260401-20260801.xml	curl 0	32946 bytes
```

### c8b
```
15:23:37	B-bills.xml	a-c8b-B-bills.xml	curl 0	2808 bytes
```

### s9
```
15:26:53	N-0201.xml	a-s9-N-0201.xml	curl 0	3022 bytes
15:26:53	N-0202.xml	a-s9-N-0202.xml	curl 0	3022 bytes
15:26:54	N-0203.xml	a-s9-N-0203.xml	curl 0	3022 bytes
15:26:54	N-0204.xml	a-s9-N-0204.xml	curl 0	3022 bytes
15:26:54	N-0205.xml	a-s9-N-0205.xml	curl 0	3022 bytes
15:26:54	N-0206.xml	a-s9-N-0206.xml	curl 0	3022 bytes
15:26:54	N-0207.xml	a-s9-N-0207.xml	curl 0	3022 bytes
15:26:54	N-0208.xml	a-s9-N-0208.xml	curl 0	3022 bytes
15:26:54	N-0209.xml	a-s9-N-0209.xml	curl 0	3022 bytes
15:26:54	N-0210.xml	a-s9-N-0210.xml	curl 0	3022 bytes
15:26:55	N-0211.xml	a-s9-N-0211.xml	curl 0	3022 bytes
15:26:55	B-bills.xml	a-s9-B-bills.xml	curl 0	46 bytes
15:26:55	S-snap-20260401-20260801.xml	a-s9-S-snap-20260401-20260801.xml	curl 0	25480 bytes
15:26:55	M-marks.xml	a-s9-M-marks.xml	curl 0	3394 bytes
```
