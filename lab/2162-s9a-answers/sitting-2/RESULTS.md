# Row 2162-S9a, second sitting: no party-type field is offered for BRIDGE Customer RJ, before or after its location is set

Run on 9 Nov 2026, 11:37Z to 11:52Z, on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, read from the file, as the Gateway shows no release number), by the sheet at `lab/2162-s9a-sheet` (e558c9f8), in a sitting of its own on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic). Every value below is M, measured by this run. The first sitting's answers are in the folder above this one.

**Result: the sheet's "no such field is offered" outcome.** The ledger was seeded as expected and read twice, equal. Its Ledger Alteration screen offers no field for the GST party type. On the run lead's decision (option c) the location was then set by itself (save A: Country India, State Rajasthan) to see whether the field appears; it does not, on the ledger screen or on the "Additional GST Details" sub-screen. So no party type was set: no SEZ save, no Deemed Export save. The book was restored from the first sitting's BK and reads as it did at s1 and s2, byte for byte.

## What save A stored
Setting Country and State (nothing else keyed) changed 19 leaves of the ledger's K read (435 to 443 leaves) and 2 of its C read (`evidence/leafdiff-s3r-s4ar-*.txt`):
- K: `ALTERID` 246 to 247; `COUNTRYOFRESIDENCE` '' to India; `PRIORSTATENAME` '' to Rajasthan; `GSTNATUREOFSUPPLY` '' to 'Not Applicable'; `LEDGERCOUNTRYISDCODE` '' to +91; `LANGUAGENAME.LIST/LANGUAGEID` 0 to 1033; `REMOTEALTGUID` '' to a GUID; a new `LEDMAILINGDETAILS.LIST` (APPLICABLEFROM 20260401, MAILINGNAME BRIDGE Customer RJ, STATE Rajasthan, COUNTRY India, PINCODE and CITY empty); a new `CONTACTDETAILS.LIST` (NAME Primary Mobile No., ISDEFAULTWHATSAPPNUM Yes, PHONENUMBER and COUNTRYISDCODE empty).
- C: `ALTERID` and `LANGUAGEID` only. The ledger's `LEDGSTREGDETAILS.LIST` (Regular, Rajasthan, one GSTIN, from 20260401) is unchanged in both reads.
- `ISSEZPARTY` is not among the leaves that differ. `GSTNATUREOFSUPPLY`, a direct child of the ledger, took the value 'Not Applicable' when Country was set; the build's compliance read does not fetch it (its C diff does not show it).

## Checks
| check | result |
|---|---|
| Sheet files against `SHA256.txt` (11 files) | PASS: all equal at e558c9f8. |
| Step 0 of C2, no company loaded | PASS: one tally.exe, which alone owns 9001; `GET /status` 200. |
| C1, load alone | PASS: `PILOT` typed at Select Company, one row, Enter, no prompt. The Gateway: BRIDGE PILOT LAB alone, 1-Apr-26 to 31-Mar-27, current date Tuesday 15-Sep-2026, date of last entry 15-Sep-26, footnote "(e) Data exceptions exist". |
| Step 1, s1 and the BK | PASS: one COMPANY row, the book's GUID, ALTVCHID 68, ALTMSTID 245, byte-equal to the first sitting's s1 (`8a0c4268…`); `count7.py --same-row` equal. So the first sitting's BK stands for step 1's, as the sheet allows: `TDBK1800_<company number>.001`, 90,051 B, sha256 `6463ce768caf83fdd13ecd3dd4807535f9502f41383bac43e20c04d4a94f771b`, re-hashed equal before use and again before the restore. No new BK. |
| Step 2, the book as found | PASS: C 18 LEDGER rows, 0 named `BRIDGE Customer RJ` (38,278 B, `4976bbec…`); K 0 LEDGER rows (2,994 B, `6e9e95e8…`). Both byte-equal to the first sitting's s2. |
| Step 3, the seed | PASS: s3m before-write check passed; `L-seed.xml` sent alone; answer 538 B, root RESPONSE, CREATED 1, every other counter 0, no LINEERROR. The window after it showed the plain Gateway, no dialog. |
| Step 3, s3r and s3b | PASS: C 19 LEDGER rows, one `BRIDGE Customer RJ` with one `LEDGSTREGDETAILS.LIST` (APPLICABLEFROM 20260401, Regular, Rajasthan, one GSTIN held); K one LEDGER row, Sundry Debtors, ISBILLWISEON No, the same list; M ALTVCHID 68, ALTMSTID 246. `--leafdiff` s3r against s3b: K 435 and 435 leaves, 0 differ; C 17 and 17, 0 differ. |
| Step 4, the party-type field | Not offered (the sheet's named outcome; details below). |
| Save A (the run lead's decision, option c) | Done: before-write check passed (s4am, 68/246); Country Not Applicable to India, State Not Applicable to Rajasthan; Ctrl+A, no prompt; s4ar M 68/247. The diff is above. |
| Save B (SEZ), save C (Deemed Export) | Not made: no party-type field after save A. s4q marks byte-equal to s4m (68/247): nothing saved. |
| Step 6, the restore | PASS: shut (no prompt); Restore from the BK folder, which listed BRIDGE PILOT LAB alone, backup 9-Nov-26 (4:58 pm); "Accept ?" y; "Company data folder (<company number>) already exists." Replace (r); no other prompt; loaded alone, no prompt. |
| Step 6, s9 | PASS: M 68/245, `--same-row` against s1 equal, byte-equal to s1; C 18 LEDGER rows, 0 named `BRIDGE Customer RJ`, byte-equal to s2; K 0 LEDGER rows, byte-equal to s2. |
| C10 | PASS: Company > Shut, BRIDGE PILOT LAB alone, no prompt; Select Company with no company loaded; one tally.exe, which alone owns 9001; `GET /status` 200. |

## Step 4, the screens
Path: Gateway, Alter (`a`), Down to Ledger, Enter, typed `Customer RJ` (one row listed), Enter: Ledger Alteration.

**As seeded.** Name BRIDGE Customer RJ; (alias) empty; Under Sundry Debtors (Current Assets); Maintain balances bill-by-bill No; Mailing Details: Name BRIDGE Customer RJ, Address empty, State Not Applicable, Country Not Applicable; Banking Details: Provide bank details No; Tax Registration Details: PAN/IT No. empty, Registration type empty, GSTIN/UIN empty; Opening Balance (on 1-Apr-26) empty. Side buttons: F10 Other Masters, I More Details, B Get HSN/SAC Info and L Fetch Details Using GSTIN/UIN (both greyed), F12 Configure. Walked with Enter only: the List of States offered only Not Applicable; the cursor went from PAN/IT No. to Opening Balance, past Registration type and GSTIN/UIN. No party-type field. The screen shows Registration type and GSTIN empty although both reads hold Regular and a GSTIN.

**After save A.** Country India showed a new Pincode field, and Tax Registration Details then read Registration type Regular, GSTIN/UIN the seed's, and a new field "Set/Alter additional GST details" (No); Registration type and GSTIN were not keyed. On reopening, the "GST Registration Types" list offered Unknown, Composition, Regular, Unregistered/Consumer and Show More. "Set/Alter additional GST details" set to Yes opened a sub-screen "Additional GST Details" with two fields only: Place of Supply (for Outwards) Rajasthan, and Is the Party a Transporter No. No party-type field there either. Both screens were left with Ctrl+Q, no prompt, and nothing was saved.

Not opened: F12 Configure and I More Details on the ledger screen. Either may hold a setting that shows more fields; opening them was not in the decision.

## Deviations
- **Esc on a master screen.** In the first walk of step 4 the Ledger Alteration screen was left with Esc and "Quit ?" answered y, then Esc twice to the Gateway (the sheet's step 4 wording). LAB-COMMON C7 says to leave a master or voucher screen with Ctrl+Q, after a capture that shows the title, never Esc. Nothing was saved: s4q read byte-equal to s4m (68/246). Every later exit used Ctrl+Q.
- **Reused labels.** Save B reused the labels s4m and s4q of that first walk, so the first walk's `a-s4m-M-marks.xml`, `a-s4q-M-marks.xml`, `count-s4m.txt` and `check-s4m.txt` were replaced. Their values (68/246, check passed; s4q byte-equal to s4m) are in `evidence/notes.txt` and `evidence/count-s4q.txt`, and both sends are in `logs/log-s4m.txt` and `logs/log-s4q.txt`.
- **Harness.** `count7.py` stopped with a Python UnicodeEncodeError printing the rupee sign to the PC's cp1252 console; the same command with `python -X utf8` printed the whole count. The pushed count outputs are those runs.

## Decides
The sheet's line "No party-type field is offered for this customer" holds, on this book and release, both before and after the ledger's location is set. Where Tally keeps the party type, and its values, are not established here. The next row's variable is a setting that shows the field: F12 Configure or More Details on the ledger screen, or a company GST feature.

## Files
- `answers/`: the marks answers, the import answer and the K answers that hold no GSTIN, byte for byte. The C answers and the K answers that hold the seed's invented GSTIN stay local, by size and sha256 (`kept-local.txt`).
- `evidence/`: the notes (UTC), each count, check and leafdiff output, the answer list and the send times. The process and port listings stay local.
- `logs/`: `run.py`'s own logs. `scripts/`: the step 0 helper, the window capture helper and the before-write helper.
- `SCREENSHOTS.txt`: the captures by name, size and sha256; they stay on the lab PC.
- `substitutions.txt`: what changed in the pushed copies of the lab's own text.
