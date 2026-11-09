# Row 2162-RP: the build's own invoice posts under an unused number; a byte-identical replay of the request that created `BP/26-27/0010` alters that voucher in place (ALTERED 1, CREATED 0, same GUID)

Run on 9 Nov 2026, 22:09:50Z to 22:21:36Z (03:39 to 03:51 IST on 10 Nov), on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, as the Gateway shows no release number), by the sheet at `lab/2162-rp-sheet` (86c0029f), on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic, in master's registry), loaded alone in a sitting of its own. Raw requests only, one at a time through `run.py`; the two imports were the only writes, and no Bridge build ran. The book was restored from the row's own backup after each part. Every value below is M, measured by this run, in one run: **partial** (one sitting, one book, one release). Tally's current period was 1-Apr-26 to 31-Mar-27 and its current date Tuesday 15-Sep-2026 throughout; neither was changed. Every read was answered, each with STATUS 1, each in under two seconds; the 5-minute wait rule was never near.

This sitting ran after row 683-TB (Tally quit at 21:39:21Z), on the run lead's word, with a Tally start of its own. Tally was started once from its desktop shortcut and closed by its own Quit at the end.

**Result.**
- **s1 and BK:** one COMPANY row, the book's GUID, ALTVCHID 68, ALTMSTID 245. BK was taken by C5; the marks after it (`s1b`) were 68/245, unchanged.
- **s2, the book before:** N-0040 0 rows (no voucher holds `BP/26-27/0040`). N-0010 one row, `BRIDGE Sales` (BRIDGEVCHISSALES Yes), GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b`. R-0010 one row: `BRIDGE Walk-in`, included Yes, uncertain No, excluded No, 6 legs, the same GUID, ALTERID 17. As EXPECT.
- **Part A, s3 (`I-0040.xml`, the build's own write, sent once): CREATED 1,** ALTERED 0, DELETED 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LASTVCHID 45, LASTMID 0; no LINEERROR. The whole window after it: the Gateway, no dialog. As EXPECT.
  - s3r: N-0040 one row, `BRIDGE Sales`. R-0040 one row: `BRIDGE Walk-in`, the 6 legs as sent (-1051.00; 600.40; 400.40; 25.02; 25.02; 0.16), included Yes, every other status flag No, GUID `...-0000002d`, ALTERID 69. N-0010 and R-0010's voucher row as at s2. ALTVCHID 69 (s3m 68), ALTMSTID 245.
  - `--diff` of R-0010 against R-0040 at s3r: ALTERID, GUID, MASTERID, REFERENCE, VOUCHERKEY, VOUCHERNUMBER and VOUCHERRETAINKEY differ; `lists compared: 1`, `legs: 6 -> 6`. Only the number and identity fields differ. As EXPECT.
- **Step 4, the restore:** s4's M byte-equal to s1's (`--same-row` equal); N-0040 0 rows; N-0040, N-0010 and R-0010 byte-equal to s2's; `--diff` s2/s4 R-0010: `lists compared: 1`, `legs: 6 -> 6`. The book was as before part A.
- **Part B, s5 (`I-0010.xml`, `0010`'s own request byte for byte, sent once): CREATED 0, ALTERED 1,** DELETED 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0, LASTVCHID 11, LASTMID 0; no LINEERROR. The whole window after it: the Gateway, no dialog.
  - s5r: N-0010 **one row**, GUID `...-0000000b` (s2's), MASTERID 11. R-0010 **one row**, the same GUID, the same 6 legs and status flags, **ALTERID 17 -> 69**. ALTVCHID 69 (s5m 68), ALTMSTID 245.
  - `--diff` s2/s5r R-0010: `ALTERID: '17' -> '69'`, `lists compared: 1`, `legs: 6 -> 6`. Nothing else in the voucher row moved.
  - Line by line, s2 against s5r, beside that ALTERID: five header counters of each answer (LEDGER, CURRENCY, TAXUNIT, VOUCHERNUMBERSERIES, VOUCHER, under the answer's own header, not the voucher) moved from 0, and in M the ALTVCHID 68 -> 69. Row 2162-HS saw the same header counters move after its import; all are 0 again after the restore (s7).
- **Step 6, the look:** the Gateway capture after s5 and at step 6 is byte-equal to the one at s1. Nothing on it changed. No key was pressed.
- **Step 7, the restore:** s7's M byte-equal to s1's (`--same-row` equal); N-0040 0 rows; N-0040, N-0010 and R-0010 byte-equal to s2's; `--diff` s2/s7 R-0010: `lists compared: 1`, `legs: 6 -> 6`. The book is as it was before the row.

**By the sheet's Decides:**
- **Part A, s3 created `0040` as sent:** measured. The build's own bytes post under an unused number, as row 2162-HS's edited request did: two runs of one shape. Item 11's first sub-item is measured, PARTIAL (one book, one release).
- **Part B, `ALTERED` 1, `CREATED` 0, one voucher under `0010` with s2's GUID, entries unchanged:** measured. A byte-identical replay is not refused on this Sales type (Manual numbering, duplicates prevented). Tally re-states the stored invoice in place: the same GUID, the same legs and flags, only its ALTERID moved to the book's next mark. No duplicate was made, no dialog came, no LINEERROR. By the sheet, item 11's "a replay is refused" then needs rewording: Tally does not refuse it, it alters.
- **`CREATED` 1, two vouchers under `0010`; a refusal in the answer; a dialog; an entry, a flag or the number changed:** not seen.

**The sheet's estimates:** every estimate held: s2 as row 2162-HS's s2; s3 CREATED 1, stored as sent, included, 6 legs; s5 ALTERED 1 and CREATED 0, every other counter 0, no LINEERROR, no dialog, one row with s2's GUID, ALTERID moved to the book's next mark (69), ALTVCHID one above s5m's; s4 and s7 equal to s2 and s1.

## Step 0 and step 1: start, check, load, BK
- Sheet files: all 15 equal to `SHA256.txt` at 86c0029f, by size and sha256. `python -I gen.py`, run in a copy, wrote `I-0010.xml`, `N-0010.xml`, `N-0040.xml` and `R-0040.xml` byte-equal to the sheet's, and found `I-0040.xml` equal to the template's two changes. `LAB-COMMON.md`, `run.py` and `count15.py` are byte-equal to row 2162-GY's. The book's GUID is in origin/master's synthetic-uuid registry and is not the excluded GUID; THE COMPANY's `Company.1800` holds the book's GUID and not the excluded GUID.
- Tally was started once, at 22:09:50Z, from its desktop shortcut (`TallyPrime.lnk`, `tally.exe` 1.1.7.2), after tasklist showed none. Its start-up company loaded by itself, as the install is set to, and was shut first by C9 (the K: Company label, Shut, Enter on it, no prompt). Three Bridge processes were running; none was ended and no Bridge tool was called (a standing rule of the lab).
- C2 step 0 at 22:10:42Z, with no company loaded: the window capture, one `tally.exe`, which alone owns 9001, and `GET /status` 200 (51 B).
- At Select Company the book's name was typed: one row, BRIDGE PILOT LAB, 1-Apr-26 to 31-Mar-27. Enter, no prompt. The Gateway showed (`s1-gateway`): BRIDGE PILOT LAB alone, TallyPrime with SILVER on its title; current period 1-Apr-26 to 31-Mar-27; current date Tuesday 15-Sep-2026; date of last entry 15-Sep-26; "(Schedule Backup)" on the company row.
- **BK** (C5):
  - Before the backup, `C:\lab\2162-rp` did not exist; `backup-1` was then made new and empty (0 entries).
  - The Data menu (by its label), Backup & Restore, Backup. As in row 2162-GY, the cursor opened on Company Name, with Backup Data Path reading `C:\`; Backspace moved it back to the path field.
  - Specify Path (Up 3 times), then `C:\lab\2162-rp\backup-1` typed, and its row taken.
  - BRIDGE PILOT LAB alone, End of List. "Accept ?" was captured (`s1-bk-accept`, 22:13:16Z) and answered y; no other message.
  - The folder then held one file: `TDBK1800_<company number>.001`, 90,051 B, sha256 `5408efc698404a37b9dfee65ef80ff3d281dffe180218fc1f278a7464cd4ea05`, written at 03:43 IST. The Restore list showed it as 10-Nov-26 (3:43 am).

## The sends
Each send's start time is in `evidence/sends-utc.txt` (UTC) and each answer's in `log-<label>.txt` (local time). Each request's hash was checked against `SHA256.txt` by `send.sh` just before it was sent. `I-0040.xml` and `I-0010.xml` were each sent once.

| Step | Label | Requests | Sent (UTC) | Answer bytes |
|---|---|---|---|--:|
| 1 | s1 | M | 22:11:17.028 | 3,394 |
| 1 | s1b | M | 22:13:44.073 | 3,394 |
| 2 | s2 | N-0040, N-0010, R-0010, M | 22:13:46.846 | 3,022 / 5,604 / 94,474 / 3,394 |
| 3 | s3m | M | 22:14:06.751 | 3,394 |
| 3 | s3 | `I-0040.xml` | 22:14:14.880 | 540 |
| 3 | s3r | N-0040, R-0040, N-0010, R-0010, M | 22:14:34.290 | 5,608 / 94,478 / 5,606 / 94,476 / 3,396 |
| 4 | s4 | M, N-0040, N-0010, R-0010 | 22:17:18.666 | 3,394 / 3,022 / 5,604 / 94,474 |
| 5 | s5m | M | 22:17:30.368 | 3,394 |
| 5 | s5 | `I-0010.xml` | 22:17:39.005 | 540 |
| 5 | s5r | N-0010, R-0010, M | 22:17:58.557 | 5,606 / 94,476 / 3,396 |
| 7 | s7 | M, N-0040, N-0010, R-0010 | 22:20:38.555 | 3,394 / 3,022 / 5,604 / 94,474 |

- Before s3 and before s5, the before-write check passed: one COMPANY row, the same GUID, 68/245 against s1's.
- After each import the whole window was captured (`after-s3` at 22:14:19.177Z, `after-s5` at 22:17:42.865Z): the Gateway, no dialog. C2's status check then passed (`s3c`, `s5c`: one `tally.exe` owning 9001, `GET /status` 200) before the next read.

## Steps 4 and 7: the restores (C6)
Each time, before the shut: the BK re-hashed equal to its record, its folder holding that one file; `restore-plan.txt` written (kept with a copy per step); THE COMPANY's data folder listed. Then:
- Shut by K: Company (clicked), h, BRIDGE PILOT LAB alone, Enter, no prompt.
- The Data menu, Backup & Restore, Restore. Restore Data Path read the Tally data folder. Specify Path (Up 3 times), `C:\lab\2162-rp\backup-1` typed. The list showed BRIDGE PILOT LAB, its number, 10-Nov-26 (3:43 am), as the plan. Its row, then End of List.
- "Accept ?" captured (`s4-restore-accept` 22:16:12.680Z; `s7-restore-accept` 22:19:34.460Z) and answered y. The prompt "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder." came with the book's own number, was captured (`s4-restore-replace` 22:16:29.900Z; `s7-restore-replace` 22:19:50.704Z) and answered r. No other prompt; Select Company came back.
- The data folder was listed again; BRIDGE PILOT LAB loaded alone (one row, Enter, no prompt); the Gateway captured (`s4-gateway`, `s7-gateway`, each byte-equal to `s1-gateway`); C2's status check passed (`s4c`, `s7c`); then the reads, MARKS first.

## Step 8: close (C10)
- At 22:20:53Z (C11, after the last request): one `tally.exe`, which alone owns 9001, and `GET /status` 200.
- Shut by K: Company (clicked), h, BRIDGE PILOT LAB alone, Enter, no prompt. Select Company with no company loaded (`c10-select-company`). At 22:21:18Z: one `tally.exe` owning 9001, and `GET /status` 200 (51 B). No XML request was sent after the shut.
- Tally was then closed by its own Quit ("Q: Quit" clicked at Select Company). No prompt appeared, and no `tally.exe` ran after.

## Deviations and notes
- **No dialog came at any point** but the C5 and C6 prompts. Go To was not used. The K: Company and Data labels were clicked, and Tally's own title bar (its empty part) was clicked before each key so that the key reached Tally.
- **The header counters.** Every answer after an import carries five counters under its own header (LEDGER, CURRENCY, TAXUNIT, VOUCHERNUMBERSERIES, VOUCHER) that read 0 before it and moved after it, as in rows 2162-HS and 2162-GY. They are not in any voucher row and were 0 again after each restore. That is why N-0010's and R-0010's sha256 at s3r and s5r differ from s2's while their voucher rows do not.
- **The staging checks.** The start-up check found no hit in the answers, the logs, the evidence or this file. Each voucher's VOUCHERKEY value goes whole in the answers, as the run lead allowed on 9 Nov; in the lab's own text the `--diff` lines show it as `<15-digit voucher key>`.

## Not measured
Every item the sheet lists as not measured. Also not measured:
- Why the replay leaves the voucher's MASTERID at 11 while moving its ALTERID; only that it does, here.
- Whether Tally's Edit Log or any other screen records the replay; no screen was opened.

## Files
- `answers/`: the answers, byte for byte, except any that the standing rules keep on the lab PC; those are listed by size and sha256 in `kept-local.txt`, with the reason.
- `logs/`: `run.py`'s own logs.
- `evidence/`: the notes (UTC), `count15.py`'s output per step, the before-write, same-row and diff outputs, the line-by-line compares of s2 with s3r and with s5r, the sha256 compares and the send times. The process and port listings and the data-folder listings stay local.
- `scripts/`: the step 0 helper, the window capture helper, the send helper and the C6 prep helper.
- `SCREENSHOTS.txt`: the captures by name, size and sha256; they stay on the lab PC.
- `substitutions.txt`: what changed in the pushed copies of the lab's own text.

## count15.py outputs, whole

### s1
```
== a-s1-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s1b
```
== a-s1b-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s2
```
== a-s2-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s2-N-0010.xml: 5604 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0010' DATE '20260801' BRIDGEVCHISSALES 'Yes'
== a-s2-N-0040.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-R-0010.xml: 94474 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0010'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0010'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b'
   ALTERID '17'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
```

### s3m
```
== a-s3m-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s3
```
== a-s3-I-0040.xml: 540 bytes
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
```

### s3r
```
== a-s3r-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s3r-N-0010.xml: 5606 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0010' DATE '20260801' BRIDGEVCHISSALES 'Yes'
== a-s3r-N-0040.xml: 5608 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0040' DATE '20260801' BRIDGEVCHISSALES 'Yes'
== a-s3r-R-0010.xml: 94476 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0010'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0010'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b'
   ALTERID '17'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
== a-s3r-R-0040.xml: 94478 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0040'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0040'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   ALTERID '69'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
```

### s4
```
== a-s4-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s4-N-0010.xml: 5604 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0010' DATE '20260801' BRIDGEVCHISSALES 'Yes'
== a-s4-N-0040.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s4-R-0010.xml: 94474 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0010'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0010'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b'
   ALTERID '17'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
```

### s5m
```
== a-s5m-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s5
```
== a-s5-I-0010.xml: 540 bytes
  root RESPONSE
  CREATED '0'
  ALTERED '1'
  DELETED '0'
  LASTVCHID '11'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
```

### s5r
```
== a-s5r-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s5r-N-0010.xml: 5606 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0010' DATE '20260801' BRIDGEVCHISSALES 'Yes'
== a-s5r-R-0010.xml: 94476 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0010'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0010'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b'
   ALTERID '69'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
```

### s7
```
== a-s7-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s7-N-0010.xml: 5604 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0010' DATE '20260801' BRIDGEVCHISSALES 'Yes'
== a-s7-N-0040.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s7-R-0010.xml: 94474 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0010'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0010'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b'
   ALTERID '17'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   leg 'BRIDGE Walk-in' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
```

## Checks and diffs, whole

### s3m against s1 (before-write)
```
== a-s3m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### R-0010 against R-0040, at s3r
```
== --diff a-s3r-R-0010.xml a-s3r-R-0040.xml
   ALTERID: '17' -> '69'
   GUID: '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b' -> '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID: '11' -> '45'
   REFERENCE: 'BP/26-27/0010' -> 'BP/26-27/0040'
   VOUCHERKEY: '<15-digit voucher key>' -> '<15-digit voucher key>'
   VOUCHERNUMBER: 'BP/26-27/0010' -> 'BP/26-27/0040'
   VOUCHERRETAINKEY: '41' -> '129'
   lists compared: 1
   legs: 6 -> 6
```

### N-0010, R-0010 and M, s2 against s3r, line by line
```
== a-s2-N-0010.xml a-s3r-N-0010.xml
  @@ -11 +11 @@
  -    <LEDGER>0</LEDGER>
  +    <LEDGER>11</LEDGER>
  @@ -19 +19 @@
  -    <CURRENCY>0</CURRENCY>
  +    <CURRENCY>1</CURRENCY>
  @@ -39 +39 @@
  -    <TAXUNIT>0</TAXUNIT>
  +    <TAXUNIT>1</TAXUNIT>
  @@ -42,2 +42,2 @@
  -    <VOUCHERNUMBERSERIES>0</VOUCHERNUMBERSERIES>
  -    <VOUCHER>0</VOUCHER>
  +    <VOUCHERNUMBERSERIES>1</VOUCHERNUMBERSERIES>
  +    <VOUCHER>2</VOUCHER>
== a-s2-R-0010.xml a-s3r-R-0010.xml
  @@ -11 +11 @@
  -    <LEDGER>0</LEDGER>
  +    <LEDGER>11</LEDGER>
  @@ -19 +19 @@
  -    <CURRENCY>0</CURRENCY>
  +    <CURRENCY>1</CURRENCY>
  @@ -39 +39 @@
  -    <TAXUNIT>0</TAXUNIT>
  +    <TAXUNIT>1</TAXUNIT>
  @@ -42,2 +42,2 @@
  -    <VOUCHERNUMBERSERIES>0</VOUCHERNUMBERSERIES>
  -    <VOUCHER>0</VOUCHER>
  +    <VOUCHERNUMBERSERIES>1</VOUCHERNUMBERSERIES>
  +    <VOUCHER>2</VOUCHER>
== a-s2-M-marks.xml a-s3r-M-marks.xml
  @@ -11 +11 @@
  -    <LEDGER>0</LEDGER>
  +    <LEDGER>11</LEDGER>
  @@ -19 +19 @@
  -    <CURRENCY>0</CURRENCY>
  +    <CURRENCY>1</CURRENCY>
  @@ -39 +39 @@
  -    <TAXUNIT>0</TAXUNIT>
  +    <TAXUNIT>1</TAXUNIT>
  @@ -42,2 +42,2 @@
  -    <VOUCHERNUMBERSERIES>0</VOUCHERNUMBERSERIES>
  -    <VOUCHER>0</VOUCHER>
  +    <VOUCHERNUMBERSERIES>1</VOUCHERNUMBERSERIES>
  +    <VOUCHER>2</VOUCHER>
  @@ -50 +50 @@
  -     <ALTVCHID TYPE="Number"> 68</ALTVCHID>
  +     <ALTVCHID TYPE="Number"> 69</ALTVCHID>
```

### M, s4 against s1 (--same-row)
```
== a-s4-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
```

### R-0010, s2 against s4
```
== --diff a-s2-R-0010.xml a-s4-R-0010.xml
   lists compared: 1
   legs: 6 -> 6
```

### sha256, s2 then s4, per answer
```
N-0040 s2 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 s4 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 EQUAL
N-0010 s2 850c2b166e02c2666a9e5c9807646ff2b9d2e8cdc68f7ff2af21b71bc45b1e85 s4 850c2b166e02c2666a9e5c9807646ff2b9d2e8cdc68f7ff2af21b71bc45b1e85 EQUAL
R-0010 s2 b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5 s4 b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5 EQUAL
```

### s5m against s1 (before-write)
```
== a-s5m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### R-0010, s2 against s5r
```
== --diff a-s2-R-0010.xml a-s5r-R-0010.xml
   ALTERID: '17' -> '69'
   lists compared: 1
   legs: 6 -> 6
```

### N-0010, R-0010 and M, s2 against s5r, line by line
```
== a-s2-N-0010.xml a-s5r-N-0010.xml
  @@ -11 +11 @@
  -    <LEDGER>0</LEDGER>
  +    <LEDGER>22</LEDGER>
  @@ -19 +19 @@
  -    <CURRENCY>0</CURRENCY>
  +    <CURRENCY>1</CURRENCY>
  @@ -39 +39 @@
  -    <TAXUNIT>0</TAXUNIT>
  +    <TAXUNIT>2</TAXUNIT>
  @@ -42,2 +42,2 @@
  -    <VOUCHERNUMBERSERIES>0</VOUCHERNUMBERSERIES>
  -    <VOUCHER>0</VOUCHER>
  +    <VOUCHERNUMBERSERIES>3</VOUCHERNUMBERSERIES>
  +    <VOUCHER>2</VOUCHER>
== a-s2-R-0010.xml a-s5r-R-0010.xml
  @@ -11 +11 @@
  -    <LEDGER>0</LEDGER>
  +    <LEDGER>22</LEDGER>
  @@ -19 +19 @@
  -    <CURRENCY>0</CURRENCY>
  +    <CURRENCY>1</CURRENCY>
  @@ -39 +39 @@
  -    <TAXUNIT>0</TAXUNIT>
  +    <TAXUNIT>2</TAXUNIT>
  @@ -42,2 +42,2 @@
  -    <VOUCHERNUMBERSERIES>0</VOUCHERNUMBERSERIES>
  -    <VOUCHER>0</VOUCHER>
  +    <VOUCHERNUMBERSERIES>3</VOUCHERNUMBERSERIES>
  +    <VOUCHER>2</VOUCHER>
  @@ -79 +79 @@
  -     <ALTERID TYPE="Number"> 17</ALTERID>
  +     <ALTERID TYPE="Number"> 69</ALTERID>
== a-s2-M-marks.xml a-s5r-M-marks.xml
  @@ -11 +11 @@
  -    <LEDGER>0</LEDGER>
  +    <LEDGER>22</LEDGER>
  @@ -19 +19 @@
  -    <CURRENCY>0</CURRENCY>
  +    <CURRENCY>1</CURRENCY>
  @@ -39 +39 @@
  -    <TAXUNIT>0</TAXUNIT>
  +    <TAXUNIT>2</TAXUNIT>
  @@ -42,2 +42,2 @@
  -    <VOUCHERNUMBERSERIES>0</VOUCHERNUMBERSERIES>
  -    <VOUCHER>0</VOUCHER>
  +    <VOUCHERNUMBERSERIES>3</VOUCHERNUMBERSERIES>
  +    <VOUCHER>2</VOUCHER>
  @@ -50 +50 @@
  -     <ALTVCHID TYPE="Number"> 68</ALTVCHID>
  +     <ALTVCHID TYPE="Number"> 69</ALTVCHID>
```

### M, s7 against s1 (--same-row)
```
== a-s7-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
```

### R-0010, s2 against s7
```
== --diff a-s2-R-0010.xml a-s7-R-0010.xml
   lists compared: 1
   legs: 6 -> 6
```

### sha256, s2 then s7, per answer
```
N-0040 s2 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 s7 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 EQUAL
N-0010 s2 850c2b166e02c2666a9e5c9807646ff2b9d2e8cdc68f7ff2af21b71bc45b1e85 s7 850c2b166e02c2666a9e5c9807646ff2b9d2e8cdc68f7ff2af21b71bc45b1e85 EQUAL
R-0010 s2 b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5 s7 b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5 EQUAL
```
