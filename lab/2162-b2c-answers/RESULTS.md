# Row 2162-B2C: Tally's GSTR-1 lists BP/26-27/0010, the build's invoice to an unregistered customer, under B2C (Small) Invoices - 7, on the Rajasthan - 5% line

Run on 9 Nov 2026, 16:14Z to 16:20Z (21:44 to 21:50 IST), on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, read from the running process's file, as the Gateway shows no release number), by the sheet at `lab/2162-b2c-sheet` (a2fd065a), on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic), loaded alone in a sitting of its own. Reads and screen reads only: nothing was sent that writes and nothing was keyed that saves; no Bridge build. Every value below is M, measured by this run, in one run: **partial** (a screen read, one invoice, one book, one release). Tally's current period was 1-Apr-26 to 31-Mar-27 and its current date Tuesday 15-Sep-2026 throughout; neither was changed. Every read was answered, each with STATUS 1.

This sitting ran between sitting 1 of row 2360-L (closed by C10 at 16:13:56Z) and that row's Tally Quit (16:20Z), on the run lead's word: each of that row's sittings restores its own BK and keeps its own baselines, so this row's reads add no variable to it, and it needed no Tally start of its own.

**Result.**
- **s2:** R-0010 one row as S13a2's `s2` read it: `BRIDGE Walk-in`, PARTYGSTIN empty, Rajasthan, `Unregistered/Consumer`, included Yes, uncertain No, excluded No, 6 legs (CGST 2.5% and SGST 2.5%). W 40 rows, 34 with BRIDGEVCHISSALES Yes and 6 with No.
- **GSTR-1** opened on 1-Aug-26 to 31-Aug-26 with no list of registrations (`08ZZZZZ0000Z1ZQ`), and no period key was pressed. Total Vouchers 39; Included in Return 27; Not Relevant for This Return 7; Uncertain Transactions (Corrections needed) 5. Return View: only **B2C (Small) Invoices - 7** holds anything: 27 (2), taxable 13,508.70, CGST 383.26, SGST 383.24, tax 766.50. B2B is empty.
- **B2C (Small) Invoices - 7** opened as a list of places of supply and rates: Rajasthan - 5% (12,808.70) and Rajasthan - 18% (700.00). **Rajasthan - 5%**, `0010`'s rate, opened the Voucher Register of 27 lines, and **`BP/26-27/0010` is on it**: 1-Aug-26, BRIDGE Walk-in, BRIDGE Sales, taxable 1,000.80, CGST 25.02, SGST 25.02, tax 50.04, invoice 1,051.00.
- **s4:** M, R-0010 and W each byte-equal to s2's; `--same-row` against s1 equal; `--diff` printed only `lists compared: 1` and `legs: 6 -> 6`. The look changed nothing.

**By the sheet's Decides:**
- `BP/26-27/0010` listed under B2C (Small): measured, on the Rajasthan - 5% line, with the read-back's amounts. With S13a2's B2B half, item 19's placement is read for both kinds of customer, PARTIAL. The rehearsal company's cause stays not isolatable on a book we hold.
- The screen's counts against W: W holds 40 rows of 2026-08 (34 Sales-class); GSTR-1's Total Vouchers is 39, Included in Return 27, Not Relevant 7 and Uncertain 5. These are recorded side by side, not reconciled: the screen does not list which voucher sits in Not Relevant or Uncertain, and those lists were not opened. `0010` is not part of any gap: it is listed as included.
- s4 unchanged: measured.

## Step 1: load and check
- Sheet files: all 8 equal to `SHA256.txt` at a2fd065a; `LAB-COMMON.md`, `run.py`, `M-marks.xml`, `R-0010.xml`, `W-2026-08.xml` and `count11.py` byte-equal to row 2162-KC's.
- C2: one `tally.exe`, which alone owns 9001; `GET /status` 200; no company loaded. THE COMPANY's `Company.1800` holds BRIDGE PILOT LAB and its GUID. Two Bridge server processes and one Bridge communication process were running; they were recorded and not ended (a standing rule of the lab; no Bridge tool was called).
- Loaded alone from Select Company (the name typed, one row, Enter, no prompt). The Gateway: BRIDGE PILOT LAB alone, TallyPrime with SILVER on its title, current period 1-Apr-26 to 31-Mar-27, current date Tuesday 15-Sep-2026, date of last entry 15-Sep-26 (`s1-gateway`).
- `s1`: one COMPANY row, `GUID is the book's: yes`, ALTVCHID 68, ALTMSTID 245.
- **BK** (C5), a fallback only: the Data menu (by its label), Backup & Restore, Backup, Specify Path to a new empty folder. BRIDGE PILOT LAB alone, End of List; "Accept ?" captured (`s1-bk-accept`) and answered y; no other message. `TDBK1800_<company number>.001`, 90,051 B, sha256 `7cf140b2e2fcf6dc0d80a9a5fecd44178669bd837fb35aadd4e012b9496e9cf5`, 21:45 IST. It was not restored: nothing changed.

## Step 2: the month as the book holds it
`s2`: R-0010, W-2026-08 and M; the outputs are below. EXPECT met.

## Step 3: Tally's GSTR-1 screen, read only
Every screen, line and key is written down in `evidence/s3-gstr1-screens.txt`; the captures are listed in `SCREENSHOTS.txt`. In short:
1. The Gateway (`s3-1-gateway`). Go To by clicking its "G: Go To" label (the sheet says Alt+G; see Deviations); `GSTR-1` typed; the List of Reports, GST Reports: GSTR-1, GSTR-1 Reconciliation, GSTR-1 Filed Summary - Downloaded, no line highlighted. One Down highlighted GSTR-1; Enter. No list of GST registrations appeared.
2. GSTR-1, GST Registration `08ZZZZZ0000Z1ZQ`, Status Not Filed, 1-Aug-26 to 31-Aug-26, with F2: Period offered and no Alt+F2. No period key was pressed.
3. The summary (`s3-3-gstr1-summary`): Total Vouchers 39; Included in Return 27 (Ready for Upload 27; Modified in Books After Upload/Export 27); Not Relevant for This Return 7; Uncertain Transactions (Corrections needed) 5; Check Vouchers Having Potential Conflicts with Masters: Yes. Of the Return View's 19 lines, only B2C (Small) Invoices - 7 holds values: Vch Count 27 (2), taxable 13,508.70, CGST 383.26, SGST/UTGST 383.24, tax 766.50; total the same. The other 18 lines, B2B Invoices - 4A, 4B, 4C, 6B, 6C among them, are empty.
4. Down 17 times to `B2C (Small) Invoices - 7`, Enter: "GSTR-1 - Summary", Details of B2C (Small) Invoices - 7 (`s3-4-b2c-small-summary`): Rajasthan - 5%, taxable 12,808.70, CGST 320.26, SGST 320.24, tax 640.50; Rajasthan - 18%, 700.00, 63.00, 63.00, 126.00. The lines are places of supply and rates, so Enter on Rajasthan - 5% (0010's CGST 2.5% plus SGST 2.5%): "GSTR-1 - Voucher Register", Vouchers of Rajasthan (5%), 27 lines (`s3-4b-rajasthan-5-vouchers`), `BP/26-27/0010` among them as above. The screen listed voucher numbers, so F5 was not pressed. Nothing was pressed on a voucher line.
5. Esc on the Voucher Register, Esc on the B2C summary. On GSTR-1, Down 9 times to show its last three lines (Amendment of Adjusted Advances - 11B, HSN Summary - 12 (B2B - B2C Supplies), Document Summary - 13), all empty (`s3-5-gstr1-lower-lines`). Esc: the Gateway (`s3-6-gateway`, byte-equal to `s1-gateway`). No prompt and no dialog appeared at any point.

## Step 4: the reads after the look
`s4`: M, R-0010 and W. `--same-row` against s1: equal. `--diff` s2 against s4 R-0010: `lists compared: 1`, `legs: 6 -> 6`, nothing else. M, R-0010 and W each byte-equal to s2's. Every EXPECT met.

## Step 5: close (C10)
Shut by K: Company (clicked), h, BRIDGE PILOT LAB alone, Enter, no prompt. Select Company with no company loaded (`c10-select-company`). One `tally.exe`, which alone owns 9001; `GET /status` 200. No XML request was sent after the shut. Tally was then closed by its own Quit as step 6 of row 2360-L's sitting 1, at 16:20Z.

## Deviations and notes
- **Go To** was opened by clicking its "G: Go To" label, not by Alt+G as the sheet says. Menus were opened by clicking their labels throughout (K: Company, Y: Data, G: Go To), as the lab's standing rule asks.
- **The sitting's place:** between row 2360-L's sitting 1 C10 and that sitting's Tally Quit, on the run lead's word (see above).
- **The Gateway** showed "(e) (Schedule Backup)" on the company row, with "(e) Data exceptions exist" below, at every capture of it.
- **GSTR-1's own F5** was "F5: Nature View" on the summary and "F5: Voucher-wise" on the B2C summary; neither was pressed.

## Not measured
Every item the sheet lists as not measured; which vouchers make GSTR-1's Not Relevant (7) and Uncertain (5) counts; the Rajasthan - 18% line's vouchers; what "(2)" beside B2C (Small)'s count of 27 counts.

## Files
- `answers/`: the answers, byte for byte, except any that the standing rules keep on the lab PC (listed by size and sha256 in `kept-local.txt`).
- `logs/`: `run.py`'s own logs. `evidence/`: the notes (UTC), each count, the `--same-row` and `--diff` outputs, the byte compares, the send times, the GSTR-1 screens written down and the BK record. The process and port listings stay local.
- `scripts/`: the step 0 helper, the window capture helper and the send helper.
- `SCREENSHOTS.txt`: the captures by name, size and sha256; they stay on the lab PC.
- `substitutions.txt`: what changed in the pushed copies of the lab's own text.

## count11.py outputs, whole

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

### s4
```
== a-s4-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
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
== a-s4-W-2026-08.xml: 115168 bytes
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

## The --same-row and --diff outputs, whole

```
== a-s4-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
== --diff a-s2-R-0010.xml a-s4-R-0010.xml
   lists compared: 1
   legs: 6 -> 6
```

### s4 against s2, byte compare
```
M-marks s4 byte-equal s2
R-0010 s4 byte-equal s2
W-2026-08 s4 byte-equal s2
```
