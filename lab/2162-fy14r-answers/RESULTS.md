# Row 2162-FY14R: TallyPrime offers no restart of numbering for a Manual series on this release

Run on 9 Nov 2026, 14:00Z to 14:12Z, on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, read from the running process's file, as the Gateway shows no release number), by the sheet at `lab/2162-fy14r-sheet` (6ed4c28e), in a sitting of its own on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic). Raw requests through `run.py` and one alteration screen opened by hand; no Bridge build. Every value below is M, measured by this run, in one run: **partial**. Tally's current period was 1-Apr-26 to 31-Mar-27 and its current date Tuesday 15-Sep-2026 throughout; neither was changed.

**Result.**
- **Step 3, the restart:** the Voucher Type Alteration screen of `BRIDGE Sales` (Method of Voucher Numbering Manual, Prevent creating duplicate Voucher Nos. Yes) has **no restart field**. More Details, with Show More and Show Inactive, lists no restart entry. F12: Configure lists no restart option. Nothing was changed or keyed; the screen was left by Ctrl+Q with no prompt, and nothing was saved.
- By the sheet's Decides: **No restart offered under Manual.** On this release a series the build admits (Manual, one series) cannot be given a restart from this screen. The build's rule for a restarting series covers a state this screen does not make; it rests on no observation.
- So step 3 sent no write and `s3r` was not read. Steps 4 and 5 were not sent (the sheet goes to step 6 in that case). Row 2162-FY14's s4 and s5 results stand as measured there, on the same book and series.
- The restore from BK reads equal to s1 (68 / 245, byte-equal), T is byte-equal to s2's, and both 2027-28 N reads have 0 rows.

## Step 1: load and check
- Sheet files: all 18 equal to `SHA256.txt` at 6ed4c28e; `LAB-COMMON.md` byte-equal to rows 2433 N's and 2162-FY14's. `gen.py`, re-run in a separate folder, rebuilt all seven generated files byte for byte (`evidence/gen-check.txt`).
- C2: one `tally.exe`, which alone owns 9001; `GET /status` 200; no company loaded. Loaded alone by Select Company (`PILOT` typed, one row, Enter, no prompt). The Gateway: BRIDGE PILOT LAB alone, current period 1-Apr-26 to 31-Mar-27, current date Tuesday 15-Sep-2026 (before 5-Apr-2027), date of last entry 15-Sep-26.
- `s1`: one COMPANY row, `GUID is the book's: yes`, ALTVCHID 68, ALTMSTID 245; byte-equal to row 2162-FY14's `s1`.
- **BK** (C5): the Data menu (by its label), Backup & Restore, Backup. The Backup Data Path was changed from C:\ by Specify Path to the new empty folder `<backup folder>` (listed empty before). BRIDGE PILOT LAB alone, End of List; "Accept ?" y; no other message. `TDBK1800_<company number>.001`, 90,051 B, sha256 `0e0752f228d386cc92640e9e050bc4ec4a1fb177caea5cc91315be5c26835d09`, 19:32 local (the Restore list showed 9-Nov-26, 7:32 pm). `s1bk` marks: byte-equal to `s1`.

## Step 2: the book as found
`s2`, every EXPECT met, and all six answers byte-equal to row 2162-FY14's `s2`:
- T: 28 `VOUCHERTYPE` rows. `BRIDGE Sales` 1 row, `PARENT` Sales, one series: `NAME` Default, `NUMBERINGMETHOD` Manual, `PREVENTDUPLICATES` Yes, `DUPLICATECONTROL` '', `BEGINNINGNUMBER` 1, and one `RESTARTFROMLIST.LIST` with 0 children.
- N-0010-y26: one row, `BP/26-27/0010`, `DATE` 20260801. N-0010-y27 and N-0071-y27: 0 rows.
- V-0010: one row, `DATE` 20260801, `BRIDGE Sales`. V-0071: 0 rows.

## Step 3: the restart, looked for on screen
- `s3m`: 68 / 245, the GUID the book's; the before-write check passed (`evidence/check-s3m.txt`).
- The path: Alter (clicked in the Gateway menu); Master Alteration, Down three times to Voucher Type, Enter; List of Voucher Types, `BRIDGE Sales` typed (BRIDGE Sales, BRIDGE Sales Two, BRIDGE Sales Z0 and BRIDGE Sales Z1 listed, none highlighted), Down once (BRIDGE Sales highlighted), Enter.
- **Voucher Type Alteration, as text:** Name BRIDGE Sales; (alias) empty. General: Select type of voucher Sales; Abbreviation Sale; Activate this Voucher Type Yes; Method of Voucher Numbering Manual; Prevent creating duplicate Voucher Nos. Yes; Show unused vch nos. in transactions for Retain Original Voucher No. behaviour No (greyed); Use effective dates for vouchers No; Allow zero-valued transactions No; Make this voucher type as 'Optional' by default No; Allow narration in voucher Yes; Provide narrations for each ledger in voucher No; Enable default accounting allocations No; WhatsApp voucher after saving No. Printing: Print voucher after saving No; Use for POS invoicing No; Default template to print Not Applicable; Default title to print empty; Default bank Not Applicable; Default jurisdiction empty; Set/alter declaration No; Set/Alter Terms and Conditions No. Name of Class: empty. No restart field.
- **I: More Details** (clicked): Type of voucher Sales; List of Voucher Type Details, General Details: Name and Alias BRIDGE Sales; Method of Voucher Numbering Manual; Use Effective date for Vouchers No; Use zero-valued transactions No; Set as Optional by default No; Use Narrations in Voucher Yes; Set default Accounting Allocations No; Set Printing Options <Value exists>; Use Narrations for each Ledger No; Use Voucher Class. Show More (Enter) added Language Alias, Abbreviation Sale and Position Index in Reports 70. Show Inactive (Down, Enter) added Allow generation of payment link/QR code No (greyed). No restart entry in any view. Left by Ctrl+Q after a capture: the alteration screen, unchanged.
- **F12: Configure** (clicked): Show more configurations No; Provide aliases for Name Yes; Provide language aliases for Name No; Set Method of Voucher Numbering Yes; Define multiple numbering series for vouchers No; Allow use of Effective date option in Voucher Type Yes; Allow use of Narrations for Vouchers Yes; Allow use of Stock Journal as Manufacturing Journal Yes; Allow tracking of Additional Costs for Purchases in Reports Yes; Allow selection of default Tax Unit Yes; Allow setting of default Accounting Allocation Yes; Allow sharing voucher through WhatsApp after saving Yes; Set Printing options Yes; Allow use of Voucher Class Yes. No restart option. Nothing changed; left by Ctrl+Q after a capture: the alteration screen.
- **Leaving:** Ctrl+Q on the alteration, after a capture: no prompt; the List of Voucher Types. Ctrl+Q after a capture: the Gateway. The method was not changed and no value was keyed. No write was sent; `s3r` was not read.
- Captures: `s3-alteration-open`, `s3-more-details`, `s3-more-details-show-more`, `s3-more-details-show-inactive`, `s3-f12-configuration`, `s3-alteration-before-quit`, `s3-list-of-voucher-types-after-quit` (`SCREENSHOTS.txt`).

## Steps 4 and 5
Not sent. The sheet's step 3 sends the row to step 6 with no write when Tally offers no restart for a Manual series.

## Step 6: the restore (C6)
- BK re-hashed equal, one file in its folder; `restore-plan.txt` written; the company's data folder listed before the restore, after it and after the close (kept local).
- Shut by K: Company (clicked), h: BRIDGE PILOT LAB alone, Enter, no prompt.
- Restore under the Data menu (by its label), Backup & Restore, Restore. The Restore Data Path was Tally's data folder, as planned. The source was Specify Path, the BK folder. The list showed one row: BRIDGE PILOT LAB, THE COMPANY's number, 9-Nov-26 (7:32 pm). It was selected alone, then End of List. The prompts, word for word:
  - "Accept ?" answered y.
  - "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder." answered r (Replace).
  - No other prompt. Tally went to Select Company.
- At Select Company, `PILOT` showed one row, BRIDGE PILOT LAB, 1-Apr-26 to 31-Mar-27. Loaded with Enter, no prompt. The Gateway showed BRIDGE PILOT LAB alone, 1-Apr-26 to 31-Mar-27, current date Tuesday 15-Sep-2026, date of last entry 15-Sep-26; no dialog.
- `s9`: M 68 / 245, `--same-row` against s1 equal, byte-equal to s1. T byte-equal to s2's (`RESTARTFROMLIST.LIST` with 0 children). N-0010-y27 and N-0071-y27: 0 rows, each byte-equal to s2's. PASS.

## Step 7: close (C10)
Shut Company by K: Company (clicked), h, BRIDGE PILOT LAB alone, Enter, no prompt. Select Company with no company loaded. One `tally.exe`, which alone owns 9001; `GET /status` 200. No XML request was sent after the shut. PASS.

## Decides
- **No restart offered under Manual.** On this release and book, the alteration screen of a Manual Sales type offers no restart: no field, no More Details entry, no F12 option. The build's planned rule for a series whose `RESTARTFROMLIST.LIST` is not empty covers a state this screen does not make for the series the build admits. That rule still fails closed, but no row has observed it.
- The sheet's other two outcomes (s5 declined or created under a restart) were not reached; the restart's effect on the duplicate guard is unmeasured.

## Deviations and notes
- **Menus** were opened by clicking their labels (K: Company, Y: Data, Alter, More Details, F12: Configure), not by Alt keys.
- **The data folder** held 23 files before the restore, 17 after it and 20 after the close. After the restore, CmpSchMetaData.1800 and five .TSF files were absent and every other file had its earlier size. After the close, the .1800 and three of the .TSF were back at their earlier sizes, three .TSF were absent, and no new file appeared.
- **The Gateway after the restore** showed "(e) (Schedule Backup)" on the company row, with "(e) Data exceptions exist" below.
- **Labels.** Each label was used once. `s1bk` is the marks read after the BK that C5 asks for; it is not in the sheet.

## Not measured
A restart under any other numbering method; whether a restart can be set another way (an import of the type, another screen or release); whether a restart scopes the duplicate guard; and every item the sheet lists as not measured.

## Files
- `answers/`: the answers, byte for byte, except any that the run lead's standing rules keep on the lab PC (listed by size and sha256 in `kept-local.txt`).
- `logs/`: `run.py`'s own logs. `evidence/`: the notes (UTC), each count and check output, the send times and the BK record. The process and port listings, the folder listings and `restore-plan.txt` stay local.
- `scripts/`: the step 0 helper, the window capture helper and the before-write helper.
- `SCREENSHOTS.txt`: the captures by name, size and sha256; they stay on the lab PC.
- `substitutions.txt`: what changed in the pushed copies of the lab's own text.
