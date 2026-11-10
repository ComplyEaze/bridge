# Row 1379-IT: the invoice request with one amount-only stock-item line was created; the build's read-back returns the line's ledger and amount as a leg, with the item held under it as an inventory allocation to which Tally added a godown and batch

Run on 10 Nov 2026, 02:20:08Z to 02:33:26Z (07:50 to 08:03 IST), by the sheet at `lab/1379-it-sheet` (bab72dc8). The book was `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic), loaded alone in a sitting of its own, on TallyPrime Silver: the Gateway on 9001, `tally.exe` file version 1.1.7.2 (read from the program's file, as the Gateway shows no release number).
- Raw reads only, one at a time through `run.py`; no Bridge build.
- Nothing was keyed in Tally but BK (C5), the stock item (step 3) and the restore (C6). The one write, `I-0093.xml`, was sent once.
- Every value below is M, measured by this run, in one run.
- Tally's current period was 1-Apr-26 to 31-Mar-27 and its current date Tuesday 15-Sep-2026 throughout; neither was changed.
- Every read was answered, each with STATUS 1.

The one GSTIN in this folder is the synthetic family's company GSTIN, `08ZZZZZ0000Z1ZQ`, as the book holds it.

**Result, by the sheet's Decides (PARTIAL: one import, one sitting, one book, one release).**
- **"Created, and R returns the line's ledger and amount among its legs."** `I-0093.xml`: `CREATED` 1, `ERRORS` 0, no LINEERROR.
  - R-0093 returns one row with five legs: `BRIDGE Walk-in` Yes -1051.00; `BRIDGE Svc 998313 5%` No 1000.80; `BRIDGE CGST 2.5%` No 25.02; `BRIDGE SGST 2.5%` No 25.02; `BRIDGE Round Off` No 0.16.
  - The item line is held under the `BRIDGE Svc 998313 5%` leg of `ALLLEDGERENTRIES.LIST`, as one `INVENTORYALLOCATIONS.LIST`: `BRIDGE Item A`, side No, `RATE` empty, `AMOUNT` 1000.80, `ACTUALQTY` and `BILLEDQTY` empty, one batch allocation. The answer holds no `ALLINVENTORYENTRIES.LIST` or `INVENTORYENTRIES.LIST`.
  - The sheet asks for the entry mode and the status flags. V-0093 reads `VCHENTRYMODE` empty: Tally filled no entry mode. R-0093 and V-0093 read included `Yes`, uncertain `No`, excluded `No`.
  - V-0093 also reads `GSTNATUREOFRETURN`, `VATPARTYTRANSRETURNNUMBER` and `VATPARTYTRANSRETURNDATE` empty, `ISINVOICE` `Yes`, and `PERSISTEDVIEW` `Invoice Voucher View`. The registration fields read `GSTREGISTRATION` `Rajasthan Registration` (no attributes), `CMPGSTIN` `08ZZZZZ0000Z1ZQ`, `CMPGSTSTATE` `Rajasthan`, `CMPGSTREGISTRATIONTYPE` `Regular`, and `PLACEOFSUPPLY` and `STATENAME` `Rajasthan`.
- **"Stored other than sent", recorded whole and not judged.** The request sent no batch and no godown. In R-0093 the line's allocation holds one `BATCHALLOCATIONS.LIST`: `GODOWNNAME` `Main Location`, `BATCHNAME` `Primary Batch`, `AMOUNT` 1000.80. The legs, sides and amounts are as sent; the two replaced service legs are one leg of their sum, as the request put the line.
- **"What Tally filled for the line's GST", recorded as found.** The allocation in R-0093 reads:
  - `GSTSOURCETYPE` `Ledger` and `GSTLEDGERSOURCE` `BRIDGE Svc 998313 5%`;
  - `HSNSOURCETYPE` `Ledger` and `HSNLEDGERSOURCE` `BRIDGE Svc 998313 5%`;
  - `GSTHSNNAME` `998313`, `GSTHSNDESCRIPTION` `Information technology consulting and support services`;
  - `GSTOVRDNTAXABILITY` `Taxable`, `GSTOVRDNTYPEOFSUPPLY` `Services`, and `GSTRATEINFERAPPLICABILITY` and `GSTHSNINFERAPPLICABILITY` `As per Masters/Company`.

The other lines do not apply here:
- The import was not refused, and R's legs hold the line's amount.
- The item master did not move at s4r: S was byte-equal to s3r's.
- R-0010 did not move (`--diff`: lists compared 1, legs 6 -> 6).

- **s1:** one COMPANY row, the book's: ALTVCHID 68, ALTMSTID 245. EXPECT met.
- **BK:** taken before anything was keyed or sent; restored at step 5.
- **s2, the book before:** every EXPECT met, so no stop.
  - V-0093 0 rows; W 40 rows, none numbered `BP/26-27/0093`; R-0093 0 rows.
  - R-0010 one row: `BRIDGE Walk-in`, included `Yes`, every other status flag `No`, 6 legs.
  - S: STOCKITEM rows 0 (the book held no stock item). G: one godown, `Main Location`. M 68/245.
- **Step 3, the stock item:** created as the sheet says, with one field differing from row 1381-M's opened screen (below). s3r: S one row named `BRIDGE Item A`, its company the book's; G as at s2; ALTMSTID 245 to 246.
- **Step 4, the import:** s4m passed the before-write check. s4 as above.
  - s4r: W 41 rows, the one new row `BP/26-27/0093`; V-0093 one row; R-0093 one row; S as at s3r.
  - ALTVCHID 68 to 69, ALTMSTID 246 unchanged.
- **Step 5, the restore:** by C6, with `restore-plan.txt` written first; the two expected prompts. s5:
  - every answer byte-equal to s2's (M, W, V-0093, R-0093, R-0010, S, G);
  - M equal to s1 by `--same-row`;
  - R-0010 `--diff`: lists compared 1, legs 6 -> 6.
  - The book is as it was found: ALTVCHID 68, ALTMSTID 245, no stock item, no `BP/26-27/0093`.

**The item's master fields** (the sheet's record):
- **As opened** (`s3-item-as-opened`), word for word:
  - Name empty; (alias) empty; Under `Primary`; Units `Not Applicable`.
  - GST applicability `Applicable`.
  - HSN/SAC Details `As per Company/Stock Group`; Source of details `Company`; HSN/SAC `998341`; Description `Geological and geophysical consulting services`.
  - GST Rate Details `As per Company/Stock Group`; Source of details `Not Available`; Taxability Type empty; GST Rate `0 %`.
  - Type of Supply `Goods`; Rate of Duty (eg 5) empty; Opening Balance Quantity, Rate, per and Value empty.
- **Against row 1381-M's opened screen,** one field differs: HSN/SAC's Source of details reads `Company` here (1381-M's: Not Available), with the HSN/SAC and description above filled. They were kept as offered, as the sheet says for every field it does not name.
- **As accepted** (`s3-item-before-accept`), word for word:
  - Name `BRIDGE Item A`; (alias) empty; Under `Primary`; Units `Not Applicable`.
  - GST applicability `Applicable`.
  - HSN/SAC Details `As per Company/Stock Group`; Source of details `Company`; HSN/SAC `998341`; Description `Geological and geophysical consulting services`.
  - GST Rate Details `As per Company/Stock Group`; Source of details `Not Available`; Taxability Type empty; GST Rate `0 %`.
  - Type of Supply `Services`; Rate of Duty (eg 5) `0` (Tally's, after Enter on the empty field); Opening Balance empty.
- **s3r's S lines,** whole in `count-s3r`:
  - PARENT ` Primary` (as printed); GUID suffix `-000000e8`; MASTERID 232; ALTERID 246.
  - `HSNDETAILS.LIST` 1: from 20260401, `As per Company/Stock Group`.
  - `GSTDETAILS.LIST` 1: from 20260401, `GSTINELIGIBLEITC` Yes, `As per Company/Stock Group`, one STATEWISEDETAILS.LIST for state `Any`. Within it: CGST, SGST/UTGST, IGST and State Cess `Based on Value` with no rate; Cess `Not Applicable`; an empty GSTSLABRATES.LIST.
  - The build's item read (`S-items.xml`) fetches no `GSTTYPEOFSUPPLY`, `GSTAPPLICABLE` or `BASEUNITS`. The item's Type of Supply, `Services`, is recorded by the screenshots only.

**Deviation from the sheet:** the sheet says "Close every other Bridge process first." Three Bridge processes were running at the start. On the run lead's standing word none was ended, and they were left running at the close. No Bridge tool was called and no request went through Bridge.

**Beyond the sheet, recorded and not judged:**
- **V-0093 holds none of the lists.** `V-0093.xml` asks for `NATIVEMETHOD *`. Its answer holds 729 plain fields and no `LEDGERENTRIES.LIST`, `ALLLEDGERENTRIES.LIST`, `ALLINVENTORYENTRIES.LIST`, `INVENTORYENTRIES.LIST` or `INVENTORYALLOCATIONS.LIST`. So V does not say where Tally keeps the line. R does: under the leg in `ALLLEDGERENTRIES.LIST`. Row 2346-CD's V read returned the same way.
- **The name was typed twice.** Key-by-key typing dropped the spaces: the field first read `BRIDGEItemA`. Before any Enter, Backspace emptied it. It was then typed again with a separate space key between the words, read `BRIDGE Item A`, and accepted.
- **The Gateway's company line carried "(e)"** with the footnote "(e) Data exceptions exist", at step 0 and after the reload at step 5 alike.

## Step 1: load, check and back up
- **Sheet files:** all 19 listed in `SHA256.txt` at bab72dc8 are equal by sha256 and size.
  - `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml`, `R-0010.xml`, `W-2026-08.xml`, `template-import-0010.utf16le.xml` and `template-full-0057.utf16le.xml` are byte-equal to row 2346-CD's.
  - `gen.py`, rerun in a scratch copy, writes `I-0093.xml`, `V-0093.xml`, `R-0093.xml`, `S-items.xml` and `G-godowns.xml` byte-equal to the sheet's.
- **Before the start:** THE COMPANY's data folder was found as the one whose `Company.1800` holds BRIDGE PILOT LAB and the book's GUID (one folder). It does not hold the excluded GUID (`evidence/bookcheck.txt`).
- **Tally was started once,** at 02:20:08Z, from its desktop shortcut, after Get-Process showed none. Its start-up company loaded by itself, as the install is set to. It was shut first by C9: the K: Company label, Shut, Enter, no prompt.
- **C2 at 02:21:06Z,** with Select Company showing: one `tally.exe`, which alone owns 9001; `GET /status` 200; no company loaded.
- **The book was loaded alone.** `BRIDGE` and ` PILOT` were typed into Select Company's name filter, which showed one row, BRIDGE PILOT LAB, 1-Apr-26 to 31-Mar-27 (`s0-select`). Enter: no prompt.
  - The Gateway (`s0-gateway`) showed the book alone, SILVER on the title.
  - Current period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026; date of last entry 15-Sep-26.
- **`s1`:** as in the Result.
- **BK** (C5), by row 2162-GY's step 1 path:
  - Before the backup, `C:\lab\1379-it\backup-1` was made new and empty (0 entries).
  - The Data menu (by its label), Backup & Restore, Backup. Backspace moved the cursor to the path field.
  - Specify Path (Up 3 times, checked). `C:\lab\1379-it\backup-1` was typed in three chunks; Tally showed it as `C:\lab\1379-It\backup-1`, its own capital. Its row was taken.
  - BRIDGE PILOT LAB alone, then End of List (Up 3 times, checked).
  - "Accept ?" was captured (`bk-accept`, 02:23:25Z) and answered y. No other message; the Gateway again.
  - The folder then held one file: `TDBK1800_<company number>.001`, 90,051 B, sha256 `a89cf218d82dc9982c6b0fb422e93fc37c5f192fde6764850c14a4b9d1d0d5ab`, written at 07:53 IST.

## Step 2: the book before
`s2` (02:23:53Z): `W-2026-08.xml V-0093.xml R-0093.xml R-0010.xml S-items.xml G-godowns.xml M-marks.xml`. Each request was checked against `SHA256.txt` just before its send. Every EXPECT was met, as in the Result.

## Step 3: the stock item
- **Step 0 of C2 at 02:24:11Z:** one `tally.exe`, the same process, owning 9001; `GET /status` 200.
- **Opening the screen:** the Gateway, Create (read), Enter: Master Creation, List of Masters. Down to Stock Item (read): seven presses, as one of the first six did not register and Stock Category was read before one more. Enter: Stock Item Creation, captured as opened.
- **The name:** typed as in "Beyond the sheet", then Enter. (alias) empty, Enter.
- **Kept as offered, each read before Enter:**
  - Under: List of Groups (Create; Primary), `Primary`.
  - Units: list Units (Create; Not Applicable), `Not Applicable`.
  - GST applicability: `Applicable` (no list shown).
  - HSN/SAC Details: List of Actions (Show More; As per Company/Stock Group; Specify Details Here; Use GST Classification), `As per Company/Stock Group`.
  - GST Rate Details: the same four rows, `As per Company/Stock Group`.
- **Type of Supply:** list Types of Supply (Capital Goods; Goods; Services), Goods offered. Down 1; `Services` was read, then Enter.
- **Rate of Duty:** empty, Enter (Tally then shows 0).
- **Opening Balance:** Quantity and Rate were not cursor stops; Value was left empty, Enter.
- **The accept:** the whole screen was captured before it (`s3-item-before-accept`, 02:27:10Z). "Accept ?" Yes or No (`s3-item-accept`, 02:27:31Z): y.
- **Leaving the screen:** Tally showed a blank Stock Item Creation. Its title was read; one Ctrl+Q went back to Master Creation with no "Quit ?" box. Escape (a menu) went to the Gateway, with no dialog (`s3-gateway`).
- **C2 at 02:28:11Z,** then `s3r` (02:28:26Z): `S-items.xml G-godowns.xml M-marks.xml`.

## Step 4: the import
- **`s4m` (02:28:45Z):** M 68/246. The before-write check against s1 passed (one COMPANY row, the same GUID). No key was pressed in Tally after it.
- **`s4` (02:28:54Z):** `I-0093.xml` sent once and answered within a second: `CREATED` 1, `ALTERED` 0, `DELETED` 0, `LASTVCHID` 45, `LASTMID` 0, `COMBINED` 0, `IGNORED` 0, `ERRORS` 0, `CANCELLED` 0, `EXCEPTIONS` 0; no LINEERROR. The window after it was the Gateway with no dialog (`s4-after-send`).
- **C2 at 02:29:12Z,** then `s4r` (02:29:17Z): `W-2026-08.xml V-0093.xml R-0093.xml R-0010.xml S-items.xml M-marks.xml`.
- **The new W row:** `BP/26-27/0093`, `BRIDGE Sales`, 20260801, `ISCANCELLED` No, `ISDELETED` No, BRIDGEVCHISSALES (the Sales class) Yes, GUID suffix `-0000002d`.
- **V-0093's row:** MASTERID 45, ALTERID 69.

## Step 5: restore (C6) and compare
- **Before the shut, at 02:30:05Z:** `prep-c6.sh 4` re-hashed BK equal to `evidence/bk.txt` and found its folder holding only that file. It wrote `restore-plan.txt` (kept on the lab PC, as it names the company number) and listed the Tally data folder (kept on the lab PC).
- **The shut:** the K: Company label, Shut, BRIDGE PILOT LAB alone, Enter: Select Company, no company loaded.
- **The restore:**
  - The Data label, Backup & Restore, Restore: Restore Companies.
  - Specify Path (Up 3, checked). `C:\lab\1379-it\backup-1` was typed in three chunks (shown `C:\lab\1379-It\backup-1`).
  - Enter showed one company row, BRIDGE PILOT LAB, its number, 10-Nov-26 (7:53 am), as the plan says (`s5-restore-list`). Enter on it; Enter on End of List.
- **The two prompts:**
  - "Accept ?" (`s5-accept`): `y`.
  - "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder.", R: Replace / S: Skip (`s5-folder-exists`): `r`.
  - The restore ran and Tally returned to Select Company with no other prompt.
- **The reload:** the book alone, by the filter (`s5-reloaded`). The Gateway was as at step 0, with the same period, date and date of last entry.
- **`s5` (02:32:27Z):** `M-marks.xml W-2026-08.xml V-0093.xml R-0093.xml R-0010.xml S-items.xml G-godowns.xml`. Every compare was equal, and every answer's sha256 equal to s2's, S and G included.

## Step 6: close (C10)
- Shut by the K: Company label, Shut, BRIDGE PILOT LAB alone, Enter, no prompt: Select Company, no company loaded (`c10-no-company`).
- C2 at 02:33:03Z, after the last request: one `tally.exe` (the same process as at step 0), which alone owns 9001; `GET /status` 200. No XML request after the shut.
- Tally was then closed by its own Quit (Q: Quit clicked at Select Company), with no prompt. Get-Process at 02:33:26Z showed no `tally.exe`. The three Bridge processes were left running (the deviation).

## Files
- `answers/`: every answer that may go, byte for byte. `kept-local.txt` lists any that stay on the lab PC, by size and sha256, with the reason.
- `evidence/`: notes, every `count18.py` output and compare, and the answers' sizes and sha256. Process and port listings, the backup-folder listings and the data-folder listing stay on the lab PC.
- `logs/`: each send's console output.
- `scripts/`: the row's helpers, with lab paths replaced.
- `SCREENSHOTS.txt`: every capture, by name, size and sha256. They stay on the lab PC.

## count18.py outputs, whole

### s1
```
== a-s1-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s2
```
== a-s2-G-godowns.xml: 4146 bytes
  STATUS '1'
  GODOWN rows: 1
   NAME 'Main Location' ; PARENT ' Primary' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000063'
== a-s2-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
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
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s2-R-0093.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s2-S-items.xml: 2998 bytes
  STATUS '1'
  STOCKITEM rows: 0 ; named 'BRIDGE Item A': 0
== a-s2-V-0093.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s2-W-2026-08.xml: 115168 bytes
  STATUS '1'
  VOUCHER rows: 40
   DATE '20260803'; VOUCHERNUMBER '1'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000c'
   DATE '20260804'; VOUCHERNUMBER '1'; VOUCHERTYPENAME 'Purchase'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000017'
   DATE '20260807'; VOUCHERNUMBER '10'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000020'
   DATE '20260807'; VOUCHERNUMBER '11'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000021'
   DATE '20260807'; VOUCHERNUMBER '12'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000022'
   DATE '20260807'; VOUCHERNUMBER '13'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000023'
   DATE '20260807'; VOUCHERNUMBER '14'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000024'
   DATE '20260803'; VOUCHERNUMBER '2'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000d'
   DATE '20260804'; VOUCHERNUMBER '2'; VOUCHERTYPENAME 'Purchase'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000018'
   DATE '20260803'; VOUCHERNUMBER '3'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000e'
   DATE '20260803'; VOUCHERNUMBER '4'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000f'
   DATE '20260807'; VOUCHERNUMBER '5'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001b'
   DATE '20260807'; VOUCHERNUMBER '6'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001c'
   DATE '20260807'; VOUCHERNUMBER '7'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001d'
   DATE '20260807'; VOUCHERNUMBER '8'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001e'
   DATE '20260807'; VOUCHERNUMBER '9'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001f'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0001'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000001'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0002'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000002'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0003'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000003'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0004'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000004'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0005'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000005'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0006'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000006'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0007'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000007'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0008'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000008'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0009'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000009'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0010'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0011'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000a'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0012'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000010'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0013'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000011'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0014'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000012'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0015'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000013'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0016'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000014'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0019'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000016'
   DATE '20260805'; VOUCHERNUMBER 'BP/26-27/0021'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001a'
   DATE '20260820'; VOUCHERNUMBER 'BP/26-27/0041'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'Yes'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000028'
   DATE '20260820'; VOUCHERNUMBER 'BP/26-27/0042'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000029'
   DATE '20260822'; VOUCHERNUMBER 'BP/26-27/0043'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002b'
   DATE '20260822'; VOUCHERNUMBER 'BP/26-27/0044'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002c'
   DATE '20260821'; VOUCHERNUMBER 'BP2/0001'; VOUCHERTYPENAME 'BRIDGE Sales Two'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002a'
   DATE '20260802'; VOUCHERNUMBER 'Z1/0001'; VOUCHERTYPENAME 'BRIDGE Sales Z1'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000015'
```

### s3r
```
== a-s3r-G-godowns.xml: 4146 bytes
  STATUS '1'
  GODOWN rows: 1
   NAME 'Main Location' ; PARENT ' Primary' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000063'
== a-s3r-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s3r-S-items.xml: 7448 bytes
  STATUS '1'
  STOCKITEM rows: 1 ; named 'BRIDGE Item A': 1
   PARENT ' Primary'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000e8'
   MASTERID '232'
   ALTERID '246'
   BRIDGECOMPANYGUID is the book's: yes
   HSNDETAILS.LIST elements: 1
     HSNDETAILS.LIST[1]/APPLICABLEFROM[1]: '20260401'
     HSNDETAILS.LIST[1]/APPLICABLEFROM[1]@TYPE: 'Date'
     HSNDETAILS.LIST[1]/SRCOFHSNDETAILS[1]: 'As per Company/Stock Group'
     HSNDETAILS.LIST[1]/SRCOFHSNDETAILS[1]@TYPE: 'String'
   GSTDETAILS.LIST elements: 1
     GSTDETAILS.LIST[1]/APPLICABLEFROM[1]: '20260401'
     GSTDETAILS.LIST[1]/APPLICABLEFROM[1]@TYPE: 'Date'
     GSTDETAILS.LIST[1]/GSTINELIGIBLEITC[1]: 'Yes'
     GSTDETAILS.LIST[1]/GSTINELIGIBLEITC[1]@TYPE: 'Logical'
     GSTDETAILS.LIST[1]/SRCOFGSTDETAILS[1]: 'As per Company/Stock Group'
     GSTDETAILS.LIST[1]/SRCOFGSTDETAILS[1]@TYPE: 'String'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/GSTSLABRATES.LIST[1]: ''
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[1]/GSTRATEDUTYHEAD[1]: 'CGST'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[1]/GSTRATEDUTYHEAD[1]@TYPE: 'String'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[1]/GSTRATEVALUATIONTYPE[1]: 'Based on Value'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[1]/GSTRATEVALUATIONTYPE[1]@TYPE: 'String'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[2]/GSTRATEDUTYHEAD[1]: 'SGST/UTGST'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[2]/GSTRATEDUTYHEAD[1]@TYPE: 'String'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[2]/GSTRATEVALUATIONTYPE[1]: 'Based on Value'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[2]/GSTRATEVALUATIONTYPE[1]@TYPE: 'String'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[3]/GSTRATEDUTYHEAD[1]: 'IGST'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[3]/GSTRATEDUTYHEAD[1]@TYPE: 'String'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[3]/GSTRATEVALUATIONTYPE[1]: 'Based on Value'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[3]/GSTRATEVALUATIONTYPE[1]@TYPE: 'String'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[4]/GSTRATEDUTYHEAD[1]: 'Cess'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[4]/GSTRATEDUTYHEAD[1]@TYPE: 'String'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[4]/GSTRATEVALUATIONTYPE[1]: 'Not Applicable'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[4]/GSTRATEVALUATIONTYPE[1]@TYPE: 'String'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[5]/GSTRATEDUTYHEAD[1]: 'State Cess'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[5]/GSTRATEDUTYHEAD[1]@TYPE: 'String'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[5]/GSTRATEVALUATIONTYPE[1]: 'Based on Value'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[5]/GSTRATEVALUATIONTYPE[1]@TYPE: 'String'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/STATENAME[1]: 'Any'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/STATENAME[1]@TYPE: 'String'
```

### s4m
```
== a-s4m-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s4
```
== a-s4-I-0093.xml: 540 bytes
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

### s4r
```
== a-s4r-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s4r-R-0010.xml: 94474 bytes
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
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s4r-R-0093.xml: 90662 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0093'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0093'
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
   leg 'BRIDGE Svc 998313 5%' | 'No' | '1000.80'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 1
     under 'BRIDGE Svc 998313 5%':
       STOCKITEMNAME 'BRIDGE Item A'; ISDEEMEDPOSITIVE 'No'; RATE ''; AMOUNT '1000.80'; ACTUALQTY ''; BILLEDQTY '' ; BATCHALLOCATIONS.LIST 1
== a-s4r-S-items.xml: 7448 bytes
  STATUS '1'
  STOCKITEM rows: 1 ; named 'BRIDGE Item A': 1
   PARENT ' Primary'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000e8'
   MASTERID '232'
   ALTERID '246'
   BRIDGECOMPANYGUID is the book's: yes
   HSNDETAILS.LIST elements: 1
     HSNDETAILS.LIST[1]/APPLICABLEFROM[1]: '20260401'
     HSNDETAILS.LIST[1]/APPLICABLEFROM[1]@TYPE: 'Date'
     HSNDETAILS.LIST[1]/SRCOFHSNDETAILS[1]: 'As per Company/Stock Group'
     HSNDETAILS.LIST[1]/SRCOFHSNDETAILS[1]@TYPE: 'String'
   GSTDETAILS.LIST elements: 1
     GSTDETAILS.LIST[1]/APPLICABLEFROM[1]: '20260401'
     GSTDETAILS.LIST[1]/APPLICABLEFROM[1]@TYPE: 'Date'
     GSTDETAILS.LIST[1]/GSTINELIGIBLEITC[1]: 'Yes'
     GSTDETAILS.LIST[1]/GSTINELIGIBLEITC[1]@TYPE: 'Logical'
     GSTDETAILS.LIST[1]/SRCOFGSTDETAILS[1]: 'As per Company/Stock Group'
     GSTDETAILS.LIST[1]/SRCOFGSTDETAILS[1]@TYPE: 'String'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/GSTSLABRATES.LIST[1]: ''
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[1]/GSTRATEDUTYHEAD[1]: 'CGST'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[1]/GSTRATEDUTYHEAD[1]@TYPE: 'String'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[1]/GSTRATEVALUATIONTYPE[1]: 'Based on Value'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[1]/GSTRATEVALUATIONTYPE[1]@TYPE: 'String'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[2]/GSTRATEDUTYHEAD[1]: 'SGST/UTGST'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[2]/GSTRATEDUTYHEAD[1]@TYPE: 'String'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[2]/GSTRATEVALUATIONTYPE[1]: 'Based on Value'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[2]/GSTRATEVALUATIONTYPE[1]@TYPE: 'String'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[3]/GSTRATEDUTYHEAD[1]: 'IGST'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[3]/GSTRATEDUTYHEAD[1]@TYPE: 'String'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[3]/GSTRATEVALUATIONTYPE[1]: 'Based on Value'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[3]/GSTRATEVALUATIONTYPE[1]@TYPE: 'String'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[4]/GSTRATEDUTYHEAD[1]: 'Cess'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[4]/GSTRATEDUTYHEAD[1]@TYPE: 'String'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[4]/GSTRATEVALUATIONTYPE[1]: 'Not Applicable'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[4]/GSTRATEVALUATIONTYPE[1]@TYPE: 'String'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[5]/GSTRATEDUTYHEAD[1]: 'State Cess'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[5]/GSTRATEDUTYHEAD[1]@TYPE: 'String'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[5]/GSTRATEVALUATIONTYPE[1]: 'Based on Value'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/RATEDETAILS.LIST[5]/GSTRATEVALUATIONTYPE[1]@TYPE: 'String'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/STATENAME[1]: 'Any'
     GSTDETAILS.LIST[1]/STATEWISEDETAILS.LIST[1]/STATENAME[1]@TYPE: 'String'
== a-s4r-V-0093.xml: 52722 bytes
  STATUS '1'
  VOUCHER rows: 1
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID '45'
   ALTERID '69'
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0093'
   VOUCHERTYPENAME 'BRIDGE Sales'
   PARTYLEDGERNAME 'BRIDGE Walk-in'
   BASICBUYERNAME 'BRIDGE Walk-in'
   PARTYMAILINGNAME 'BRIDGE Walk-in'
   CONSIGNEEMAILINGNAME 'BRIDGE Walk-in'
   CONSIGNEESTATENAME 'Rajasthan'
   CONSIGNEECOUNTRYNAME 'India'
   VATDEALERTYPE 'Regular'
   COUNTRYOFRESIDENCE 'India'
   PARTYGSTIN ''
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   GSTREGISTRATION 'Rajasthan Registration'
   CMPGSTIN '08ZZZZZ0000Z1ZQ'
   CMPGSTSTATE 'Rajasthan'
   CMPGSTREGISTRATIONTYPE 'Regular'
   PLACEOFSUPPLY 'Rajasthan'
   STATENAME 'Rajasthan'
   REFERENCE 'BP/26-27/0093'
   REFERENCEDATE '20260801'
   GSTNATUREOFRETURN ''
   VATPARTYTRANSRETURNNUMBER ''
   VATPARTYTRANSRETURNDATE ''
   ISINVOICE 'Yes'
   VCHENTRYMODE ''
   PERSISTEDVIEW 'Invoice Voucher View'
   GSTREGISTRATION attributes {}
   LEDGERENTRIES.LIST elements: 0
   ALLLEDGERENTRIES.LIST elements: 0
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   direct children: 729
== a-s4r-W-2026-08.xml: 117984 bytes
  STATUS '1'
  VOUCHER rows: 41
   DATE '20260803'; VOUCHERNUMBER '1'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000c'
   DATE '20260804'; VOUCHERNUMBER '1'; VOUCHERTYPENAME 'Purchase'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000017'
   DATE '20260807'; VOUCHERNUMBER '10'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000020'
   DATE '20260807'; VOUCHERNUMBER '11'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000021'
   DATE '20260807'; VOUCHERNUMBER '12'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000022'
   DATE '20260807'; VOUCHERNUMBER '13'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000023'
   DATE '20260807'; VOUCHERNUMBER '14'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000024'
   DATE '20260803'; VOUCHERNUMBER '2'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000d'
   DATE '20260804'; VOUCHERNUMBER '2'; VOUCHERTYPENAME 'Purchase'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000018'
   DATE '20260803'; VOUCHERNUMBER '3'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000e'
   DATE '20260803'; VOUCHERNUMBER '4'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000f'
   DATE '20260807'; VOUCHERNUMBER '5'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001b'
   DATE '20260807'; VOUCHERNUMBER '6'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001c'
   DATE '20260807'; VOUCHERNUMBER '7'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001d'
   DATE '20260807'; VOUCHERNUMBER '8'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001e'
   DATE '20260807'; VOUCHERNUMBER '9'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001f'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0001'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000001'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0002'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000002'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0003'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000003'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0004'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000004'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0005'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000005'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0006'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000006'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0007'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000007'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0008'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000008'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0009'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000009'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0010'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0011'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000a'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0012'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000010'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0013'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000011'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0014'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000012'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0015'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000013'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0016'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000014'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0019'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000016'
   DATE '20260805'; VOUCHERNUMBER 'BP/26-27/0021'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001a'
   DATE '20260820'; VOUCHERNUMBER 'BP/26-27/0041'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'Yes'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000028'
   DATE '20260820'; VOUCHERNUMBER 'BP/26-27/0042'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000029'
   DATE '20260822'; VOUCHERNUMBER 'BP/26-27/0043'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002b'
   DATE '20260822'; VOUCHERNUMBER 'BP/26-27/0044'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002c'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0093'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   DATE '20260821'; VOUCHERNUMBER 'BP2/0001'; VOUCHERTYPENAME 'BRIDGE Sales Two'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002a'
   DATE '20260802'; VOUCHERNUMBER 'Z1/0001'; VOUCHERTYPENAME 'BRIDGE Sales Z1'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000015'
```

### s5
```
== a-s5-G-godowns.xml: 4146 bytes
  STATUS '1'
  GODOWN rows: 1
   NAME 'Main Location' ; PARENT ' Primary' ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000063'
== a-s5-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s5-R-0010.xml: 94474 bytes
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
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s5-R-0093.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s5-S-items.xml: 2998 bytes
  STATUS '1'
  STOCKITEM rows: 0 ; named 'BRIDGE Item A': 0
== a-s5-V-0093.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s5-W-2026-08.xml: 115168 bytes
  STATUS '1'
  VOUCHER rows: 40
   DATE '20260803'; VOUCHERNUMBER '1'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000c'
   DATE '20260804'; VOUCHERNUMBER '1'; VOUCHERTYPENAME 'Purchase'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000017'
   DATE '20260807'; VOUCHERNUMBER '10'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000020'
   DATE '20260807'; VOUCHERNUMBER '11'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000021'
   DATE '20260807'; VOUCHERNUMBER '12'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000022'
   DATE '20260807'; VOUCHERNUMBER '13'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000023'
   DATE '20260807'; VOUCHERNUMBER '14'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000024'
   DATE '20260803'; VOUCHERNUMBER '2'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000d'
   DATE '20260804'; VOUCHERNUMBER '2'; VOUCHERTYPENAME 'Purchase'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000018'
   DATE '20260803'; VOUCHERNUMBER '3'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000e'
   DATE '20260803'; VOUCHERNUMBER '4'; VOUCHERTYPENAME 'Receipt'; BRIDGEVCHISSALES 'No'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000f'
   DATE '20260807'; VOUCHERNUMBER '5'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001b'
   DATE '20260807'; VOUCHERNUMBER '6'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001c'
   DATE '20260807'; VOUCHERNUMBER '7'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001d'
   DATE '20260807'; VOUCHERNUMBER '8'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001e'
   DATE '20260807'; VOUCHERNUMBER '9'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001f'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0001'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000001'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0002'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000002'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0003'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000003'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0004'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000004'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0005'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000005'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0006'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000006'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0007'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000007'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0008'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000008'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0009'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000009'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0010'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b'
   DATE '20260801'; VOUCHERNUMBER 'BP/26-27/0011'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000a'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0012'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000010'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0013'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000011'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0014'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000012'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0015'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000013'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0016'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000014'
   DATE '20260802'; VOUCHERNUMBER 'BP/26-27/0019'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000016'
   DATE '20260805'; VOUCHERNUMBER 'BP/26-27/0021'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000001a'
   DATE '20260820'; VOUCHERNUMBER 'BP/26-27/0041'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'Yes'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000028'
   DATE '20260820'; VOUCHERNUMBER 'BP/26-27/0042'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000029'
   DATE '20260822'; VOUCHERNUMBER 'BP/26-27/0043'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002b'
   DATE '20260822'; VOUCHERNUMBER 'BP/26-27/0044'; VOUCHERTYPENAME 'BRIDGE Sales'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002c'
   DATE '20260821'; VOUCHERNUMBER 'BP2/0001'; VOUCHERTYPENAME 'BRIDGE Sales Two'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002a'
   DATE '20260802'; VOUCHERNUMBER 'Z1/0001'; VOUCHERTYPENAME 'BRIDGE Sales Z1'; BRIDGEVCHISSALES 'Yes'; ISCANCELLED 'No'; ISDELETED 'No' ; IRN none ; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000015'
```

## Compares and checks, whole

### s4m: M against s1 (the before-write check)
```
== a-s4m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s4r: R-0010 against s2 (--diff)
```
== --diff a-s2-R-0010.xml a-s4r-R-0010.xml
   lists compared: 1
   legs: 6 -> 6
```

### s5: M against s1 (--same-row)
```
== a-s5-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
```

### s5: R-0010 against s2 (--diff)
```
== --diff a-s2-R-0010.xml a-s5-R-0010.xml
   lists compared: 1
   legs: 6 -> 6
```

### sha256 of every answer
```
$ sha256sum and size of every answer, in send order
a-s1-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s2-W-2026-08.xml 115168 B f67cf46f57e7f4ae65d864f7fc73ed3b20db7027d7fbf048f1730cc691387e97
a-s2-V-0093.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-R-0093.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-R-0010.xml 94474 B b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5
a-s2-S-items.xml 2998 B cf34fa0a514109d2691ae8ccd750fd83338927b5153541d55f810b0530658a1a
a-s2-G-godowns.xml 4146 B 0dce15fd6de5a85bf40ab69585851df1f80ed3ba7a7673e2e05e898fb1aa465c
a-s2-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s3r-S-items.xml 7448 B c0dded6c2e751c46c54c9c745b67adc4b2783a0986bd2f6059d6a23258c03020
a-s3r-G-godowns.xml 4146 B e620f4504c0d1f58a90223e7b6f5232018cc11ec9b9571e11fd22ac2fd90f3ae
a-s3r-M-marks.xml 3394 B be99d683817e28bd1d592f1272be47571babebbb4467de34b33a4fbf7fe5b65f
a-s4m-M-marks.xml 3394 B be99d683817e28bd1d592f1272be47571babebbb4467de34b33a4fbf7fe5b65f
a-s4-I-0093.xml 540 B fdcd8435c6fcf95ab624cbe2cff2c887078df500626f7c9c37cfa834a1cdb6ea
a-s4r-W-2026-08.xml 117984 B d1ef5b8989c707254cf319a1477ab9bed38ea91119d09a0236b43c86b9a74fa5
a-s4r-V-0093.xml 52722 B 27e4d671ef4f10e0a666b538b089905f6cd0bf3f3bcd1385e7d0f64897f0dbbe
a-s4r-R-0093.xml 90662 B dff8c724dd8c70811e99340e38fe003331a2c8a8569ed01d30a9d1d04aa4de0a
a-s4r-R-0010.xml 94474 B 73e14c2ed28b64507d23309d1ed9f065616b143bea297beb83ac8a2cba4b1ec6
a-s4r-S-items.xml 7448 B 43e42d976507f6f45edd7a92777620298df373dcc15750fdc6c12b65bd188bcf
a-s4r-M-marks.xml 3394 B 4663f964caa68dc18dee2d93403b127fbbc5c7b17d8c29a0a0ac35cdd5b95ba5
a-s5-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s5-W-2026-08.xml 115168 B f67cf46f57e7f4ae65d864f7fc73ed3b20db7027d7fbf048f1730cc691387e97
a-s5-V-0093.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s5-R-0093.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s5-R-0010.xml 94474 B b32ec79cf0ee63c3635e94196ab2b81670f2d13e5542673753906c2bb3c2ede5
a-s5-S-items.xml 2998 B cf34fa0a514109d2691ae8ccd750fd83338927b5153541d55f810b0530658a1a
a-s5-G-godowns.xml 4146 B 0dce15fd6de5a85bf40ab69585851df1f80ed3ba7a7673e2e05e898fb1aa465c
```
