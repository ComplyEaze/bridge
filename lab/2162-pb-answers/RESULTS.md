# Row 2162-PB: S13a2's invoice to a registered customer, its GSTIN under another PAN part, is stored as sent, reads included and not uncertain, and GSTR-1 counts it under B2B

Run on 9 Nov 2026, 16:24:55Z to 16:36:15Z (21:54 to 22:06 IST), on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, read from the running process's file, as the Gateway shows no release number), by the sheet at `lab/2162-pb-sheet` (291302b1), on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic), loaded alone in a sitting of its own. Raw requests through `run.py`; no Bridge build. Every value below is M, measured by this run, in one run: **partial** (a raw import, not a build; one invoice, one book, one release). Tally's current period was 1-Apr-26 to 31-Mar-27 and its current date Tuesday 15-Sep-2026 throughout; neither was changed. Every read was answered, each with STATUS 1. Each write was sent once.

The buyer's GSTIN is written here, as everywhere in this folder, as `08<PAN-B>3ZN`: `08`, the PAN part the run lead sent (kept on the lab PC), `3Z` and the check character N that `gen10.py` printed. Every answer that holds the PAN part stays on the lab PC (`kept-local.txt`).

This sitting ran in the gap between sitting 1 of row 2360-L (Tally quit at 16:20Z) and that row's sitting 2 (after 00:00 IST on 10 Nov), on the run lead's word: Tally was started for it once from its desktop shortcut and closed by its own Quit at the end; the book was restored, so the marks it leaves are those it found.

**Result.**
- **s3:** `L-seedB.xml` answered CREATED 1 and every other counter 0. The ledger reads as seeded: C one row, `BRIDGE Customer RJ`, Sundry Debtors, ISBILLWISEON No; Q one registration row that holds anything: APPLICABLEFROM 20260401, Regular, Rajasthan, GSTIN `08<PAN-B>3ZN`.
- **s4:** `I-0053B.xml` answered CREATED 1, LASTVCHID 45, every other counter 0, no LINEERROR, no dialog. R-0053 one row: `BRIDGE Customer RJ`, PARTYGSTIN `08<PAN-B>3ZN` (the GSTIN sent), Regular, Rajasthan, ISINVOICE Yes, **included Yes, uncertain No, excluded No**, not overridden, not post-dated, 6 legs as sent. N-0053 one row, Sales class. Marks 68/245 to 69/246.
- **The diff against S13a2's `0053`** printed one field, `PARTYGSTIN: '08ZZZZZ0000Z3ZO' -> '08<PAN-B>3ZN'`, then `lists compared: 1` and `legs: 6 -> 6`. The identity fields (GUID, ALTERID, MASTERID, VOUCHERKEY, VOUCHERRETAINKEY) equal S13a2's: both rows start from the same restored book and send the same files but for the GSTIN.
- **GSTR-1** opened on 1-Aug-26 to 31-Aug-26 with no list of registrations (`08ZZZZZ0000Z1ZQ`); no period key was pressed. Total Vouchers 40; Included in Return 28 (Ready for Upload 28: Not Uploaded 1, Modified in Books After Upload/Export 27); Not Relevant 7; Uncertain 5. **B2B Invoices - 4A, 4B, 4C, 6B, 6C:** 1, taxable 1,000.80, CGST 25.02, SGST 25.02, tax 50.04, invoice 1,051.00. Opened, it lists one party line: `08<PAN-B>3ZN` (the GSTIN sent), BRIDGE Customer RJ, 1 voucher, the same amounts.
- **The look changed nothing:** `--same-row` s4r/s5m equal; `--diff` s4r/s5r printed only `lists compared: 1` and `legs: 6 -> 6`; R-0053 byte-equal.
- **Restore:** by C6. s9: M byte-equal to s1 (68/245) and C byte-equal to s2 (no `BRIDGE Customer RJ`).

**By the sheet's Decides:**
- **Included Yes, uncertain No, and B2B:** measured. Item 18's first element set is stored as the build writes it and listed as included, for a registered customer that is not bill-wise and whose GSTIN is under another PAN part. Item 19's B2B placement for that customer: one party line under B2B Invoices - 4A, 4B, 4C, 6B, 6C. Both PARTIAL: a raw import, not a build; one book, one release. S13a2's same-PAN result is therefore not a special case of the PAN.
- `PARTYGSTIN` and `GSTREGISTRATIONTYPE` read as sent: the read-back comparison would admit this post.
- s5m and s5r unchanged: measured.
- No decline, no dialog.
- Beside row 2162-B2C's look at this book without `0053` (Total 39, Included 27, B2B empty, B2C (Small) 27 (2), 13,508.70), Total and Included each rose by one and B2B holds this invoice alone; B2C (Small), Not Relevant and Uncertain are unchanged. Ready for Upload now shows a "Not Uploaded 1" line. These are recorded side by side, not reconciled.

## Step 0: the filled requests
- Sheet files: all 16 equal to `SHA256.txt` at 291302b1.
- The PAN part the run lead sent was put alone on one line in `buyer-pan.local`. `python -I gen10.py` printed:
  - `dd288e36c3de811e0c082746e5a7f12bf9d0c4a798bacc3fcce134aaaad7e7ea  1444  L-seedB.xml`
  - `5f5c8f15c8c7fbb7b6e5ef9ce0f393c31eec8c9dabdbb4deaa57c537bdb9c131  4862  I-0053B.xml`
  - `check character: N`
- `filled-sha256.local.txt` holds the same two lines. `count12.py` without `buyer-pan.local` printed stop and nothing else (exit 1). `--holds-pan` marked both filled requests as staying on the lab PC. `git status` in the answers worktree lists none of the four lab-PC files; `.gitignore` names them.

## Step 1: load and check
- C2 step 0 with no tally.exe running. Tally was started once, at 16:24:55Z, from its desktop shortcut (`TallyPrime.lnk`, `C:\Program Files\TallyPrime\tally.exe`, 1.1.7.2). Its start-up company loaded by itself, as the install is set to, and was shut first by C9 (the K: Company label, Shut, Enter on it, no prompt).
- C2 at 16:25:39Z: one `tally.exe`, which alone owns 9001; `GET /status` 200; no company loaded. THE COMPANY's `Company.1800` holds BRIDGE PILOT LAB and its GUID. No Bridge process was ended and no Bridge tool was called (a standing rule of the lab).
- Loaded alone from Select Company (the name typed, one row, Enter, no prompt). The Gateway: BRIDGE PILOT LAB alone, TallyPrime with SILVER on its title, current period 1-Apr-26 to 31-Mar-27, current date Tuesday 15-Sep-2026 (not before 1-Aug-2026), date of last entry 15-Sep-26 (`s1-gateway`).
- `s1`: one COMPANY row, `GUID is the book's: yes`, ALTVCHID 68, ALTMSTID 245.
- **BK** (C5): the Data menu (by its label), Backup & Restore, Backup, Specify Path to a new empty folder. BRIDGE PILOT LAB alone, End of List; "Accept ?" captured (`s1-bk-accept`) and answered y; no other message. `TDBK1800_<company number>.001`, 90,051 B, sha256 `e49978241024e68797500fdce8352727e0ef46fa98f932f287f5c2f4a935426f`, 21:58 IST.

## Step 2: the book as found
`s2`: C, Q, N-0053 and R-0010. 0 rows named `BRIDGE Customer RJ` in C (18 ledgers) and in Q; N-0053 0 rows; R-0010 one row (`BP/26-27/0010`, `BRIDGE Walk-in`, included Yes). EXPECT met.

## Step 3: the setup ledger
`s3m`, then `<the before-write option>` against s1: one COMPANY row, the same GUID, "the write may be sent". No key was pressed in Tally since. `s3`: `L-seedB.xml` sent once (hash equal to step 0's): CREATED 1, every other counter 0. The window after it (`s3-after-write`): the Gateway, no dialog. `s3r`: C, Q and M as in the Result; ALTMSTID 246. EXPECT met.

## Step 4: the invoice
`s4m`, then `<the before-write option>` against s1: one COMPANY row, the same GUID, "the write may be sent" (ALTMSTID 246, the ledger's). No key was pressed in Tally since. `s4`: `I-0053B.xml` sent once (hash equal to step 0's): CREATED 1, LASTVCHID 45, every other counter 0, no LINEERROR. The window after it (`s4-after-write`): the Gateway, no dialog. `s4r`: R-0053, N-0053 and M as in the Result. The two diffs are below, whole. EXPECT met.

## Step 5: Tally's GSTR-1 screen, read only
Every screen, line and key is written down in `evidence/s5-gstr1-screens.txt`; the captures are listed in `SCREENSHOTS.txt`. In short:
1. The Gateway (`s5-1-gateway`). Go To by clicking its "G: Go To" label (the sheet says Alt+G; see Deviations); `GSTR-1` typed; the List of Reports, GST Reports: GSTR-1, GSTR-1 Reconciliation, GSTR-1 Filed Summary - Downloaded, no line highlighted. One Down highlighted GSTR-1; Enter. No list of GST registrations appeared.
2. GSTR-1, GST Registration `08ZZZZZ0000Z1ZQ`, Status Not Filed, 1-Aug-26 to 31-Aug-26, with F2: Period offered and no Alt+F2. No period key was pressed. The summary (`s5-2-gstr1-summary`) as in the Result; of the Return View's lines shown, B2B Invoices - 4A, 4B, 4C, 6B, 6C and B2C (Small) Invoices - 7 hold values, every other line shown is empty.
3. Step 5.3 does not apply: `0053` reads included.
4. Down 8 times (Uncertain Transactions and Check Vouchers Having Potential Conflicts with Masters passed, neither opened) to `B2B Invoices - 4A, 4B, 4C, 6B, 6C`; Enter: "GSTR-1 - Summary", Details of B2B Invoices - 4A, 4B, 4C, 6B, 6C (`s5-4-b2b-summary`), one line: `08<PAN-B>3ZN` (the GSTIN sent), BRIDGE Customer RJ, Voucher Count 1, taxable 1,000.80, CGST 25.02, SGST/UTGST 25.02, tax 50.04, invoice 1,051.00. The line is a party, not a voucher: F5: Voucher-wise was offered and not pressed; nothing was pressed on the line.
5. Esc on the B2B summary, Esc on GSTR-1: the Gateway (`s5-5-gateway`, byte-equal to `s5-1-gateway`). No prompt and no dialog appeared at any point.
6. `s5m`: `--same-row` s4r/s5m equal. EXPECT met.
7. `s5r`: `--diff` s4r/s5r printed only `lists compared: 1` and `legs: 6 -> 6`; the answer is byte-equal to s4r's. EXPECT met.

## Step 6: the restore (C6)
Before the shut: BK re-hashed, equal to step 1's, its folder holding only that file; the restore plan written (16:32:57Z); the data folder listed (24 entries). Shut by K: Company (clicked), h, BRIDGE PILOT LAB alone, Enter, no prompt. The Data menu (by its label), Backup & Restore, Restore, Specify Path to the BK's folder: the list showed BRIDGE PILOT LAB, its number and 9-Nov-26 (9:58 pm), the BK's time. The company line, End of List. Two prompts, the only ones answered:
- "Accept ?" (`s6-restore-accept`, 16:34:24Z): y.
- "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder." (`s6-restore-replace`, 16:34:43Z): r.

Select Company came back with no other message; the data folder then held 17 entries. Loaded alone (the name typed, one row, Enter, no prompt; `s9-gateway`). `s9`: M byte-equal to s1's and `--same-row` s1/s9 equal (68/245); C byte-equal to s2's, 18 ledgers, 0 named `BRIDGE Customer RJ`. EXPECT met.

## Step 7: close (C10)
Shut by K: Company (clicked), h, BRIDGE PILOT LAB alone, Enter, no prompt. Select Company with no company loaded (`c10-select-company`). One `tally.exe`, which alone owns 9001; `GET /status` 200; the data folder 20 entries. No XML request was sent after the shut. Tally was then closed by its own Quit ("Q: Quit" clicked at Select Company) at about 16:36:15Z (22:06 IST): no prompt appeared, and at 16:36:22Z no `tally.exe` ran and nothing listened on 9001.

## Step 8: the screen before the push
`count12.py --holds-pan` was run on every answer, every evidence and notes file, the logs, the scripts and RESULTS.md; its output, whole, is `evidence/holds-pan.txt` in the pushed folder. Each answer it marked is kept on the lab PC by size and sha256 (`kept-local.txt`); no text file was marked.

## Deviations and notes
- **Go To** was opened by clicking its "G: Go To" label, not by Alt+G as the sheet says. Menus were opened by clicking their labels throughout (K: Company, Y: Data, G: Go To), as the lab's standing rule asks.
- **The sitting's place:** a Tally start of its own in the gap before row 2360-L's sitting 2, on the run lead's word; the install's start-up company was shut by C9 before anything else.
- **The identity fields** of `0053` (GUID, ALTERID, MASTERID, VOUCHERKEY, VOUCHERRETAINKEY) equal S13a2's; the sheet expected that they might or might not, and records them.
- **The Gateway** showed "(e) (Schedule Backup)" on the company row, with "(e) Data exceptions exist" below, at every capture of it.
- **GSTR-1's Return View** showed a down mark with "4" below its last line shown (Adjustment of Advances - 11B(1), 11B(2)); it was not scrolled, as the sheet asks for no more than the summary and the section that counts `0053`.
- **GSTINs in this folder:** the buyer's only as `08<PAN-B>3ZN`. The synthetic family's two, the company's own `08ZZZZZ0000Z1ZQ` and S13a2's customer's `08ZZZZZ0000Z3ZO`, appear in the lab's text as in the sheet's; answers that hold any GSTIN stay on the lab PC.

## Not measured
Every item the sheet lists as not measured; which vouchers make GSTR-1's Not Relevant (7) and Uncertain (5) counts; the Return View's lines below the ones shown; the voucher-wise view of the B2B list; what "(2)" beside B2C (Small)'s count of 27 counts.

## Files
- `answers/`: the answers, byte for byte, except any that the standing rules keep on the lab PC (listed by size and sha256 in `kept-local.txt`, with the reason).
- `logs/`: `run.py`'s own logs. `evidence/`: the notes (UTC), each count, the `<the before-write option>`, `--same-row` and `--diff` outputs, the send times, the GSTR-1 screens written down, the BK record, step 0's output and the `--holds-pan` output. The process, port and data-folder listings and the restore plan stay local.
- `scripts/`: the step 0 helper, the window capture helper and the send helper.
- `SCREENSHOTS.txt`: the captures by name, size and sha256; they stay on the lab PC.
- `substitutions.txt`: what changed in the pushed copies of the lab's own text.
- Not in this folder and never staged: `buyer-pan.local`, `filled-sha256.local.txt`, `L-seedB.xml` and `I-0053B.xml` (step 0 lists the two requests by sha256 and size).

## count12.py outputs, whole

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
== a-s2-C-catalogue.xml: 25010 bytes
  STATUS '1'
  LEDGER rows: 18; named 'BRIDGE Customer RJ': 0
== a-s2-N-0053.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-Q-rates.xml: 82392 bytes
  STATUS '1'
  LEDGER rows: 18; named 'BRIDGE Customer RJ': 0
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
== a-s3-L-seedB.xml: 538 bytes
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
```

### s3r
```
== a-s3r-C-catalogue.xml: 26224 bytes
  STATUS '1'
  LEDGER rows: 19; named 'BRIDGE Customer RJ': 1
   PARENT 'Sundry Debtors'; ISBILLWISEON 'No'
== a-s3r-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s3r-Q-rates.xml: 85190 bytes
  STATUS '1'
  LEDGER rows: 19; named 'BRIDGE Customer RJ': 1
   PARENT 'Sundry Debtors'; PARTYGSTIN <absent>; STATENAME <absent>
   LEDGSTREGDETAILS.LIST: APPLICABLEFROM '20260401'; GSTREGISTRATIONTYPE 'Regular'; PLACEOFSUPPLY 'Rajasthan'; GSTIN '08<PAN-B>3ZN'
   LEDGSTREGDETAILS.LIST elements: 1 of which non-empty: 1
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
== a-s4-I-0053B.xml: 540 bytes
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
== a-s4r-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s4r-N-0053.xml: 5844 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0053' DATE '20260801' BRIDGEVCHISSALES 'Yes'
== a-s4r-R-0053.xml: 94496 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0053'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0053'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Customer RJ'
   PARTYGSTIN '08<PAN-B>3ZN'
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Regular'
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
   leg 'BRIDGE Customer RJ' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
```

### s5m
```
== a-s5m-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s5r
```
== a-s5r-R-0053.xml: 94496 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0053'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0053'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Customer RJ'
   PARTYGSTIN '08<PAN-B>3ZN'
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Regular'
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
   leg 'BRIDGE Customer RJ' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
```

### s9
```
== a-s9-C-catalogue.xml: 25010 bytes
  STATUS '1'
  LEDGER rows: 18; named 'BRIDGE Customer RJ': 0
== a-s9-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
```

## The <the before-write option>, --same-row and --diff outputs, whole

### s3
```
== a-s3m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s4
```
== a-s4m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s4r against the control R-0010 (s2)
```
== --diff a-s2-R-0010.xml a-s4r-R-0053.xml
   ALTERID: '17' -> '69'
   GSTREGISTRATIONTYPE: 'Unregistered/Consumer' -> 'Regular'
   GUID: '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000000b' -> '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID: '11' -> '45'
   PARTYGSTIN: '' -> '08<PAN-B>3ZN'
   PARTYLEDGERNAME: 'BRIDGE Walk-in' -> 'BRIDGE Customer RJ'
   REFERENCE: 'BP/26-27/0010' -> 'BP/26-27/0053'
   VOUCHERKEY: '<15-digit voucher key>' -> '<15-digit voucher key>'
   VOUCHERNUMBER: 'BP/26-27/0010' -> 'BP/26-27/0053'
   VOUCHERRETAINKEY: '41' -> '129'
   list ALLLEDGERENTRIES.LIST: 6 -> 6, 1 differ
   lists compared: 1
   legs: 6 -> 6
   leg 1: ["'BRIDGE Walk-in'", "'Yes'", "'-1051.00'"] -> ["'BRIDGE Customer RJ'", "'Yes'", "'-1051.00'"]
```

### s4r against S13a2's 0053
```
== --diff S13A2-a-s4r-R-0053.xml a-s4r-R-0053.xml
   PARTYGSTIN: '08ZZZZZ0000Z3ZO' -> '08<PAN-B>3ZN'
   lists compared: 1
   legs: 6 -> 6
```

### s5m same-row against s4r
```
== a-s5m-M-marks.xml against a-s4r-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'246'")
  equal
```

### s5r diff against s4r
```
== --diff a-s4r-R-0053.xml a-s5r-R-0053.xml
   lists compared: 1
   legs: 6 -> 6
```

### s9 same-row against s1
```
== a-s9-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
```
