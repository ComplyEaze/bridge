# Row 2162-FY14: Tally declines, in its answer, an invoice number the book holds in 2026-27 when it is sent dated in 2027-28

Run on 9 Nov 2026, 13:21Z to 13:29Z, on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, read from the running process's file, as the Gateway shows no release number), by the sheet at `lab/2162-fy14-sheet` (e5d30d23), in a sitting of its own on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic). Raw requests only, through `run.py`; no Bridge build, no screen keying. Every value below is M, measured by this run, in one run: **partial**. Tally's current period was 1-Apr-26 to 31-Mar-27 and its current date Tuesday 15-Sep-2026 throughout; neither was changed.

**Result.**
- **s4, the control:** `BP/26-27/0071`, dated 20270405 (after the current period and the current date), was **created**: CREATED 1, every other counter 0 but LASTVCHID 45, no `LINEERROR`. The N read over 2027-28 and the V read show it, dated 20270405, type `BRIDGE Sales`. So Tally takes an invoice dated in the next financial year by import on this book.
- **s5, the variable:** `BP/26-27/0010`, which the book holds dated 20260801 in 2026-27, sent dated 20270405, was **declined in the answer**: CREATED 0, EXCEPTIONS 1, every other counter 0 (LASTVCHID 0), no `LINEERROR`. The answer is the counters alone. No dialog: the capture after it shows the plain Gateway. Nothing was created. N-0010-y26 and V-0010 read as at s2, N-0010-y27 has 0 rows, and the marks' COMPANY row equals s4r's.
- By the sheet's Decides: **Tally's `PREVENTDUPLICATES` spans financial years** on a series with no restart entry, and the build's per-year number read would admit a number Tally declines. The failure is in-band (P7), in the answer and with no dialog. Partial.
- The restore from BK reads equal to s1 (68 / 245, byte-equal), and both 2027-28 N reads are back to 0 rows.

## Step 1: load and check
- Sheet files: all 18 equal to `SHA256.txt` at e5d30d23; `LAB-COMMON.md` byte-equal to S9b's. `gen.py`, re-run in a separate folder, rebuilt all seven generated files byte for byte. `I-0071-y27.xml` and `I-0010-y27.xml` are the same length and differ in 5 characters; neither holds `20260801`.
- C2: one `tally.exe`, which alone owns 9001; `GET /status` 200; no company loaded. Loaded alone by Select Company (`PILOT` typed, one row, Enter, no prompt). The Gateway: BRIDGE PILOT LAB alone, current period 1-Apr-26 to 31-Mar-27, current date Tuesday 15-Sep-2026 (before 5-Apr-2027), date of last entry 15-Sep-26.
- `s1`: one COMPANY row, the GUID, ALTVCHID 68, ALTMSTID 245; byte-equal to S9b's `s1`.
- **BK** (C5): Data > Backup & Restore > Backup. The Backup Data Path was changed from C:\ by Specify Path to the new empty folder `<backup folder>` (listed empty before). BRIDGE PILOT LAB alone; "Accept ?" y; no other message. `TDBK1800_<company number>.001`, 90,051 B, sha256 `e21ebc7224537b4227ebb997c9ae8897ae5cd4c5e131d512478e5e63611b87ec`, 18:54 local (the Restore list showed 9-Nov-26, 6:54 pm). The listing after showed the one file. `s1bk` marks: byte-equal to `s1`.

## Step 2: the book as found
`s2`, every EXPECT met:
- T: 28 `VOUCHERTYPE` rows. `BRIDGE Sales` 1 row, `PARENT` Sales, one series: `NAME` Default, `NUMBERINGMETHOD` Manual, `PREVENTDUPLICATES` Yes, `DUPLICATECONTROL` '', `BEGINNINGNUMBER` 1, and one `RESTARTFROMLIST.LIST` with 0 children.
- N-0010-y26: one row, `BP/26-27/0010`, `DATE` 20260801, `BRIDGEVCHISSALES` Yes. N-0010-y27 and N-0071-y27: 0 rows.
- V-0010: one row, `DATE` 20260801, `BRIDGE Sales`, party `BRIDGE Walk-in`. V-0071: 0 rows.

## Step 4: the control
- `s4m`: 68 / 245; the before-write check passed (one COMPANY row, the GUID).
- `s4`: `I-0071-y27.xml`, sent once. CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 45, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0; no `LINEERROR`.
- The capture after it (`s4-after-write`): the plain Gateway, no dialog. Date of last entry still 15-Sep-26.
- `s4r`: N-0071-y27 one row, `BP/26-27/0071`, `DATE` 20270405, `BRIDGEVCHISSALES` Yes. V-0071 one row, `DATE` 20270405, `VOUCHERNUMBER` `BP/26-27/0071`, `BRIDGE Sales`, party `BRIDGE Walk-in`. M: 69 / 245.

## Step 5: the number held in 2026-27, dated in 2027-28
- `s5m`: 69 / 245, byte-equal to `s4r`'s marks; the before-write check passed.
- `s5`: `I-0010-y27.xml`, sent once. CREATED 0, ALTERED 0, DELETED 0, LASTVCHID 0, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, **EXCEPTIONS 1**; no `LINEERROR`. The answer holds the counters and nothing else.
- The capture after it (`s5-after-write`): the plain Gateway, no dialog.
- `s5r`: N-0010-y26 one row as at s2 (`BP/26-27/0010`, 20260801); N-0010-y27 0 rows; V-0010 one row as at s2 (20260801, `BRIDGE Sales`). M: 69 / 245, and `count7.py --same-row` of `s4r` against `s5r`: equal.
- Against s2 (N and V) and s4r (M), only `CMPINFO` count lines differ (`evidence/s5r-byte-diffs.txt`); the rows are equal. LAB-COMMON C4: those lines are not counts of anything read.

## Step 6: the restore (C6)
- BK re-hashed equal, one file in its folder; `restore-plan.txt` written; the company's data folder listed before and after (kept local).
- Shut by Alt+K, h: BRIDGE PILOT LAB alone, Enter, no prompt.
- Restore under Data, Backup & Restore, Restore. The Restore Data Path was Tally's data folder, as planned. The source was Specify Path, the BK folder. The list showed one row: BRIDGE PILOT LAB, THE COMPANY's number, 9-Nov-26 (6:54 pm). It was selected alone, then End of List. The prompts, word for word:
  - "Accept ?" answered y.
  - "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder." answered r (Replace).
  - No other prompt. Tally went to Select Company.
- At Select Company, `PILOT` showed one row, BRIDGE PILOT LAB, with the period **1-Apr-26 to 31-Mar-28** (1-Apr-26 to 31-Mar-27 at step 1). Loaded with Enter, no prompt. The Gateway showed BRIDGE PILOT LAB alone, 1-Apr-26 to 31-Mar-27, current date Tuesday 15-Sep-2026, date of last entry 15-Sep-26; no dialog.
- `s9`: M 68 / 245, `--same-row` against s1 equal, byte-equal to s1. N-0010-y27 and N-0071-y27: 0 rows, each byte-equal to s2's. PASS.

## Step 7: close (C10)
Shut Company by Alt+K, h (Company > Shut), BRIDGE PILOT LAB alone, Enter, no prompt. Select Company with no company loaded. One `tally.exe`, which alone owns 9001; `GET /status` 200. No XML request was sent after the shut. PASS.

## Decides
- **s5 declined in the answer, no dialog.** On this book and release, a Sales type's `PREVENTDUPLICATES` Yes, on a series with no restart entry, declines a number held in an earlier financial year. The build reads the number over the invoice's own year only (`financial_year_window`), so it would show the approval for `BP/26-27/0010` dated 2027-04-05, and Tally would then decline it with nothing created. Item 14 becomes a build change: the number read covers every year the series runs, or the build refuses a number held in any year, before the approval is shown.
- **The decline's shape** is EXCEPTIONS 1 with no `LINEERROR` and every other counter 0. A reader that keys on `LINEERROR` or ERRORS would not see it; one that keys on CREATED 0 would.

## Deviations and notes
- **Select Company after the restore** listed the book with the period 1-Apr-26 to 31-Mar-28. That is the range after s4's voucher, not the restored one; the Gateway and every read after the load showed the restored book. As in row 2433 (9 Nov), Select Company's list did not show the restored folder's own values.
- **The data folder** held 24 files before the restore and 23 after.
- **Labels.** Each label was used once. `s1bk` is the marks read after the BK that C5 asks for; it is not in the sheet.
- **The Gateway's date of last entry** stayed 15-Sep-26 after the voucher dated 20270405 was created.

## Not measured
A series with a restart entry; a number held in a later year sent dated in an earlier one; a number held by a cancelled voucher; another Sales type; an invoice built and posted through Bridge; an invoice dated before the current date in the next year; whether EXCEPTIONS 1 comes with any message in Tally's own log; any other book or release.

## Files
- `answers/`: the answers, byte for byte, except any that the run lead's standing rules keep on the lab PC (listed by size and sha256 in `kept-local.txt`).
- `logs/`: `run.py`'s own logs. `evidence/`: the notes (UTC), each count and check output, the send times and the byte-diff of s5r. The process and port listings, the folder listings and `restore-plan.txt` stay local.
- `scripts/`: the step 0 helper, the window capture helper and the before-write helper.
- `SCREENSHOTS.txt`: the captures by name, size and sha256; they stay on the lab PC.
- `substitutions.txt`: what changed in the pushed copies of the lab's own text.
