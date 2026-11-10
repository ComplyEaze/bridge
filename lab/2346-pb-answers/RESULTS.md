# Row 2346-PB: the one-party bills read told every state: the settled bill had no row, the part note left -525.50, and the over-settled bill and the note's New Ref each read as a credit row 1051.00 that tied to the closing

Run on 10 Nov 2026, 05:34:54Z to 05:48:32Z (11:04 to 11:18 IST), on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, as rows 2346-AR and 2377-NB read it from the program's file; the Gateway shows no release number), by the sheet at `lab/2346-pb-sheet` (53540665), on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic), loaded alone in a sitting of its own. Seven imports, each file sent once; read-only reads between them, one at a time through `run.py` (curl, each answer's headers kept); no Bridge build. Then the restore of this sitting's own backup. Every value below is M, measured by this run, in one run. Every read was answered, each within a few seconds; no answer was empty, so the sheet's empty-report exception was never needed. The window after each import was the Gateway, no dialog.

This sitting ran after row 2162-ST (Tally quit at 05:25:24Z), with a Tally start of its own, so step 1's "otherwise" applied: step 0 of C2, then the book loaded alone.

## Result

PARTIAL by the sheet's Decides: one book, one release, one sitting, the read sent in master's bytes and order by curl, without the reads around it.

| import | step | `CREATED` | `ERRORS` | `EXCEPTIONS` | `LASTVCHID` | LINEERROR |
|---|---|---|---|---|---|---|
| `L-ledger.xml`, `BRIDGE Walk-in BC` | s3 | 1 | 0 | 0 | 0 | none |
| `I-0099.xml`, `BP/26-27/0099` | s4 | 1 | 0 | 0 | 45 | none |
| `I-0101.xml`, `BP/26-27/0101` | s4b | 1 | 0 | 0 | 46 | none |
| `I-CN-1.xml`, c1 against `BP/26-27/0099` | s5 | 1 | 0 | 0 | 47 | none |
| `I-CN-P.xml`, p, half against `BP/26-27/0101` | s6 | 1 | 0 | 0 | 48 | none |
| `I-CN-2.xml`, c2 against `BP/26-27/0099` again | s7 | 1 | 0 | 0 | 49 | none |
| `I-CN-3.xml`, c3 naming `BP/26-27/0098` | s8 | 1 | 0 | 0 | 50 | none |

`ALTERED`, `DELETED`, `LASTMID`, `COMBINED`, `IGNORED` and `CANCELLED` were 0 in all seven answers. The notes took Credit Note numbers 1, 2, 3 and 4 in the order sent, as the Book's numbers bullet expects.

**The one-party read, in master's order, after each step.** In every state the two snapshot answers were byte-equal, the two report answers were byte-equal, each answer was HTTP 200 with one `Content-Type` (`text/xml; charset=utf-16`), and `pb.py --outcome` gave `told`.

| after | the report's rows (`BILLREF`, `BILLCL`; each dated 1-Aug-26) | sum of `BILLCL` | snapshot closing | Bills Receivable | Bills Payable | `pb.py --outcome` |
|---|---|---|---|---|---|---|
| s4, the set-up | `BP/26-27/0099` -1051.00; `BP/26-27/0101` -1051.00 | -2102.00 | -2102.00 | 0099 -1051.00; 0101 -1051.00 | none | told |
| s5, c1 | `BP/26-27/0101` -1051.00 (no row for 0099) | -1051.00 | -1051.00 | 0101 -1051.00 | none | told |
| s6, p | `BP/26-27/0101` -525.50 | -525.50 | -525.50 | 0101 -525.50 | none | told |
| s7, c2 | `BP/26-27/0099` 1051.00; `BP/26-27/0101` -525.50 | 525.50 | 525.50 | 0101 -525.50 | 0099 1051.00 | told |
| s8, c3 | `BP/26-27/0099` 1051.00; `BP/26-27/0101` -525.50; `BP/26-27/0098` 1051.00 | 1576.50 | 1576.50 | 0101 -525.50 | 0099 1051.00; 0098 1051.00 | told |

No row in any report answer was On Account; `BILLOVERDUE` was 0 on every row. Report answers: 1742 B (s4), 896 B (s5), 1282 B (s6), 2906 B (s7), 3720 B (s8). Snapshot answers: 26728 B (s4, s5), 26724 B (s6 to s8).

By the sheet's Decides, quoted for each state:
- **Every state, told:** "master's one-party read reads this state. Its rows are what the note builder's refusals are judged on (the capture-6 comment's checks 1 to 3)."
- **s5, the settled bill, no row:** "a settled bill is absent from the read. The refusal of a bill not in the read (check 1) also refuses it." "Either way, the builder refuses c2 before the post."
- **s6, the part note:** -525.50, as estimated. "the rest shown as the bill's `BILLCL` is what check 3 (the amount fits) is measured against."
- **s7 and s8, credit rows that tie:** "master tells customers holding a credit bill, and check 2 refuses an `Agst Ref` to one."

**The notes as stored (each as row 2346-AR measured).**
- s5r R-CN1: Credit Note `1`, GUID tail `0000002f`; its `BRIDGE Walk-in BC` leg one bill `BP/26-27/0099`, `Agst Ref`, 1051.00.
- s6r R-CN2: Credit Note `2`, GUID tail `00000030`; legs 525.50, -300.20, -200.20, -12.51, -12.51, -0.08; one bill `BP/26-27/0101`, `Agst Ref`, 525.50.
- s7r R-CN3: Credit Note `3`, GUID tail `00000031`; one bill `BP/26-27/0099`, `Agst Ref`, 1051.00.
- s8r R-CN4: Credit Note `4`, GUID tail `00000032`; one bill `BP/26-27/0098`, `New Ref`, 1051.00.
- No note moved an invoice or an earlier note: each `count20.py --diff` of R-0099, R-0101 and R-CN1 against its last earlier answer gave legs 6 -> 6 and no list that differs; GUID and ALTERID unchanged.
- The marks: ALTVCHID 70 after the set-up, then 71, 72, 73 and 74 after each note; ALTMSTID 246 from the ledger on.

**The way back (step 9).** Restored by C6 with only the two expected prompts. s9, every file of s2 in s2's order: `--same-row` against s1 equal (ALTVCHID 68, ALTMSTID 245); each N and R read 0 rows; B-bills and B-payable 0 groups for `BRIDGE Walk-in BC`; the snapshot no row for it; each of the twelve answers' sha256 equal to s2's (`evidence/compare-s9.txt`).

**The sheet's estimates:** s2, s4, s5, s6, s7, s8 and s9 each as estimated. At s5, of the two forms the sheet named for the settled bill, it was "no row".

**Deviations from the sheet and the lab's rules:**
1. **Bridge processes.** Three Bridge processes were running at the start. On the run lead's standing word none was ended, and they were left running at the close. No Bridge tool was called and no request went through Bridge.
2. **The start-up company.** Tally's start-up company loaded by itself, as the install is set to. It was shut by the K: Company label, Shut, Enter on it, before step 0, on the run lead's standing practice for these sittings, with no change to it. One Enter there was refused by the desktop-control tool because the desktop client had the keyboard focus; no key reached Tally. A click on Tally's title bar and a capture showed the screen unchanged before the Enter was pressed.

**Beyond the sheet, recorded and not judged:**
- Bills Receivable and Bills Payable answered 46 B, an empty `ENVELOPE`, when they held no group (s2, s9; Bills Payable at s4 to s6). s2's Bills Receivable answer is byte-equal to row 2346-AR's s2 answer.
- The snapshot answered 25480 B at s2 and s9 (no `BRIDGE Walk-in BC` row; `pb.py` counted 18 `LEDGER` rows, and the text holds 19 `<LEDGER` elements in all), 26728 B with the customer's row at s4 and s5, and 26724 B from s6 on (19 rows each, from s4 on).
- The Gateway's company line carried "(e)" and "(Schedule Backup)" at step 1.

## Step 1: load, check and back up
- Sheet files: all 33 equal to `SHA256.txt` at 53540665 (sizes too). The 19 files the sheet names as row 2346-AR's are byte-equal to AR's; `run.py` and the two `template-nb-` files are byte-equal to row 2377-NB's `run.py`, `L-lo-rj-20260804.xml` and `S-snap-20260401-20260804.xml`; `pb.py` differs from NB's in its PB-marked lines and its header comments only. `gen.py` was run again in a new folder with copies of its 17 inputs: the eight files it writes are byte-equal to the sheet's.
- Before the start, the book's data folder was found as the one whose `Company.1800` holds BRIDGE PILOT LAB and the book's GUID (one folder); it does not hold the excluded GUID (`evidence/bookcheck.txt`). The backup folder was made new and empty.
- Tally was started once, at 05:34:54Z, from its desktop shortcut, after Get-Process showed none. Its start-up company was shut first (deviation 2).
- C2 at 05:36:12Z, with Select Company showing: one `tally.exe`, which alone owns 9001; `GET /status` 200 (51 B); no company loaded.
- The book was loaded alone: `BRIDGE`, ` PILOT` and ` LAB` typed into Select Company's name filter, one row, BRIDGE PILOT LAB, 1-Apr-26 to 31-Mar-27 (`s1-select`), Enter, no prompt. The Gateway (`s1-gateway`): current period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026 (not before 1-Aug-2026); date of last entry 15-Sep-26.
- `s1`: one COMPANY row, the book's GUID, ALTVCHID 68, ALTMSTID 245. EXPECT met.
- BK by C5 (05:37Z to 05:38Z) to `C:\lab\2346-pb\backup-1` (Tally showed `C:\lab\2346-Pb\backup-1`, as the sheet says it may): Data label, Backup & Restore, Backup; Backspace to the path; Specify Path; the path typed in two chunks; Enter, Enter; `BRIDGE`, ` PILOT` and ` LAB` typed, the one row, Enter; End of List, Enter; "Accept ?" (`bk-accept`, 05:38:23Z) = y. One file, 90,051 B; the Restore list showed it as 10-Nov-26 (11:08 am).

## Step 2: the book before
- `s2` (05:38:52Z): each N and R read 0 rows; B-bills and B-payable 0 groups; the snapshot no row for `BRIDGE Walk-in BC`; M as s1. EXPECT met.

## Step 3: the set-up
- `s3m` and the before-write check against s1 passed (`evidence/bw-s3m.txt`).
- `s3` (05:39:33Z), `s4` (05:39:41Z) and `s4b` (05:39:58Z): as in the Result; each window captured at once (`s3-after`, `s4-after`, `s4b-after`): the Gateway, no dialog.
- BR at s4, with `N-0099.xml N-0101.xml R-0099.xml R-0101.xml` in `s4r`: each N one row (`BRIDGEVCHISSALES` Yes); R-0099 one row, GUID tail `0000002d`, its `BRIDGE Walk-in BC` leg one bill `BP/26-27/0099`, `New Ref`, -1051.00; R-0101 one row, GUID tail `0000002e`, one bill `BP/26-27/0101`, `New Ref`, -1051.00; B-bills two groups for `BRIDGE Walk-in BC`, as in the Result. The set-up holds.

## Steps 4 to 7: the notes
- Before each note, `s<n>m` and the before-write check against s1 passed (`evidence/bw-s5m.txt` to `bw-s8m.txt`); the note was sent alone, and the window captured at once (`s5-after` to `s8-after`): the Gateway, no dialog, each time.
- After each, BR: `<step>sa`, `<step>sb`, `<step>la` (the window captured at once, `<step>la-after`), `<step>lb`, then `<step>r` with the sheet's R reads: s5r `R-CN1.xml R-0099.xml R-0101.xml`; s6r `R-CN2.xml R-CN1.xml R-0101.xml`; s7r `R-CN3.xml R-CN1.xml R-0099.xml`; s8r `R-CN4.xml R-CN1.xml R-0099.xml`.
- p took `2`, so the sheet's step 5 fallback (R-CN2 in s7r, R-CN3 in s8r) did not apply.
- The `--diff` outputs are in `evidence/diff-s<n>r-<file>.txt`; the outcomes in `evidence/outcome-s<n>.txt`.

## Step 9: restore (C6) and compare
- `prep-c6.sh 9` first: BK re-hashed equal to `evidence/bk.txt`, its folder holding only that file; `restore-plan.txt` written; the data folder listed (kept on the lab PC).
- K: Company label, Shut, Enter on BRIDGE PILOT LAB, no prompt; Data label, Backup & Restore, Restore; Specify Path; the path typed in two chunks; the list (`restore-list`) showed one row, BRIDGE PILOT LAB, 10-Nov-26 (11:08 am), as the plan; Enter on it; End of List, Enter; "Accept ?" (`restore-accept`, 05:46:23Z) = y; "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder." (`restore-replace`, 05:46:40Z) = r. No other message.
- The book loaded alone (`s9-gateway`): period, date and last entry as at step 1.
- `s9` (05:47Z): as in the Result (`evidence/sr-s9.txt`, `evidence/compare-s9.txt`).

## Step 10: close (C10)
- Shut by the K: Company label, Shut, BRIDGE PILOT LAB alone, Enter, no prompt: Select Company, no company loaded (`c10-no-company`, 05:48:12Z).
- C2 at 05:48:13Z, after the last request: one `tally.exe` (the same process as at step 0), which alone owns 9001; `GET /status` 200. No XML request after the shut.
- Tally was then closed by its own Quit (Q: Quit clicked at Select Company), no prompt; Get-Process then showed no `tally.exe` (05:48:32Z). The three Bridge processes were left running (deviation 1).

## Files
- `answers/`: every answer and its headers file that may go, byte for byte (`kept-local.txt` lists those that stay on the lab PC, by size and sha256, with the reason).
- `evidence/`: notes, every `count20.py` and `pb.py` output, the outcomes, compares and diffs, `br.sh`'s console for s5 to s8 (s4's went to the screen only; each of its parts is in the per-label files), the answers' sizes and sha256. Process and port listings, the backup-folder listings and the data-folder listings stay on the lab PC; the BK record (`bk.txt`) goes with the company number replaced.
- `logs/`: `run.py`'s log of each send.
- `scripts/`: the row's helpers, with lab paths replaced.
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
== a-s2-N-0099.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-N-0101.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s2-R-0099.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s2-R-0101.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s2-R-CN1.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s2-R-CN2.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s2-R-CN3.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s2-R-CN4.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s2-B-bills.xml: 46 bytes
  STATUS <absent>
  BILLFIXED groups: 0
  groups for 'BRIDGE Walk-in BC': 0
== a-s2-B-payable.xml: 46 bytes
  STATUS <absent>
  BILLFIXED groups: 0
  groups for 'BRIDGE Walk-in BC': 0
== a-s2-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s3m
```
== a-s3m-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s3
```
== a-s3-L-ledger.xml: 538 bytes
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

### s4
```
== a-s4-I-0099.xml: 540 bytes
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

### s4b
```
== a-s4b-I-0101.xml: 540 bytes
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

### s4r
```
== a-s4r-B-bills.xml: 970 bytes
  STATUS <absent>
  BILLFIXED groups: 2
  groups for 'BRIDGE Walk-in BC': 2
   BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '-1051.00'
   BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-1051.00'
== a-s4r-B-payable.xml: 46 bytes
  STATUS <absent>
  BILLFIXED groups: 0
  groups for 'BRIDGE Walk-in BC': 0
== a-s4r-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s4r-N-0099.xml: 5608 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0099' DATE '20260801' BRIDGEVCHISSALES 'Yes'
== a-s4r-N-0101.xml: 5608 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0101' DATE '20260801' BRIDGEVCHISSALES 'Yes'
== a-s4r-R-0099.xml: 95834 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0099'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
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
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0099'; BILLTYPE 'New Ref'; AMOUNT '-1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '2'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s4r-R-0101.xml: 95834 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0101'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0101'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e'
   ALTERID '70'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0101'; BILLTYPE 'New Ref'; AMOUNT '-1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '3'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
exit 0
```

### s5m
```
== a-s5m-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s5
```
== a-s5-I-CN-1.xml: 540 bytes
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

### s5r
```
== a-s5r-B-bills.xml: 510 bytes
  STATUS <absent>
  BILLFIXED groups: 1
  groups for 'BRIDGE Walk-in BC': 1
   BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-1051.00'
== a-s5r-B-payable.xml: 46 bytes
  STATUS <absent>
  BILLFIXED groups: 0
  groups for 'BRIDGE Walk-in BC': 0
== a-s5r-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '71' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s5r-R-CN1.xml: 95834 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f'
   ALTERID '71'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0099'; BILLTYPE 'Agst Ref'; AMOUNT '1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '2'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s5r-R-0099.xml: 95834 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0099'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
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
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0099'; BILLTYPE 'New Ref'; AMOUNT '-1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '2'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s5r-R-0101.xml: 95834 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0101'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0101'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e'
   ALTERID '70'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0101'; BILLTYPE 'New Ref'; AMOUNT '-1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '3'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
exit 0
```

### s6m
```
== a-s6m-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '71' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s6
```
== a-s6-I-CN-P.xml: 540 bytes
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

### s6r
```
== a-s6r-B-bills.xml: 508 bytes
  STATUS <absent>
  BILLFIXED groups: 1
  groups for 'BRIDGE Walk-in BC': 1
   BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50'
== a-s6r-B-payable.xml: 46 bytes
  STATUS <absent>
  BILLFIXED groups: 0
  groups for 'BRIDGE Walk-in BC': 0
== a-s6r-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '72' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s6r-R-CN2.xml: 95830 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '2'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0101'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000030'
   ALTERID '72'
   VCHGSTSTATUSISINCLUDED 'No'
   VCHGSTSTATUSISUNCERTAIN 'Yes'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'No' | '525.50'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-300.20'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-200.20'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-12.51'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-12.51'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.08'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0101'; BILLTYPE 'Agst Ref'; AMOUNT '525.50'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '3'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s6r-R-CN1.xml: 95834 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f'
   ALTERID '71'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0099'; BILLTYPE 'Agst Ref'; AMOUNT '1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '2'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s6r-R-0101.xml: 95834 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0101'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0101'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e'
   ALTERID '70'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0101'; BILLTYPE 'New Ref'; AMOUNT '-1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '3'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
exit 0
```

### s7m
```
== a-s7m-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '72' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s7
```
== a-s7-I-CN-2.xml: 540 bytes
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

### s7r
```
== a-s7r-B-bills.xml: 508 bytes
  STATUS <absent>
  BILLFIXED groups: 1
  groups for 'BRIDGE Walk-in BC': 1
   BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50'
== a-s7r-B-payable.xml: 508 bytes
  STATUS <absent>
  BILLFIXED groups: 1
  groups for 'BRIDGE Walk-in BC': 1
   BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00'
== a-s7r-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '73' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s7r-R-CN3.xml: 95836 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '3'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000031'
   ALTERID '73'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0099'; BILLTYPE 'Agst Ref'; AMOUNT '1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '2'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s7r-R-CN1.xml: 95836 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f'
   ALTERID '71'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0099'; BILLTYPE 'Agst Ref'; AMOUNT '1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '2'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s7r-R-0099.xml: 95836 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0099'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
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
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0099'; BILLTYPE 'New Ref'; AMOUNT '-1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '2'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
exit 0
```

### s8m
```
== a-s8m-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '73' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s8
```
== a-s8-I-CN-3.xml: 540 bytes
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

### s8r
```
== a-s8r-B-bills.xml: 508 bytes
  STATUS <absent>
  BILLFIXED groups: 1
  groups for 'BRIDGE Walk-in BC': 1
   BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50'
== a-s8r-B-payable.xml: 966 bytes
  STATUS <absent>
  BILLFIXED groups: 2
  groups for 'BRIDGE Walk-in BC': 2
   BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00'
   BILLREF 'BP/26-27/0098' BILLDATE '1-Aug-26' BILLCL '1051.00'
== a-s8r-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '74' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s8r-R-CN4.xml: 95836 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '4'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000032'
   ALTERID '74'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0098'; BILLTYPE 'New Ref'; AMOUNT '1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '4'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s8r-R-CN1.xml: 95836 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f'
   ALTERID '71'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0099'; BILLTYPE 'Agst Ref'; AMOUNT '1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '2'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s8r-R-0099.xml: 95836 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0099'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
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
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0099'; BILLTYPE 'New Ref'; AMOUNT '-1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '2'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
exit 0
```

### s9
```
== a-s9-N-0099.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-N-0101.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0 of which with a VOUCHERNUMBER: 0 without one: 0
== a-s9-R-0099.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s9-R-0101.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s9-R-CN1.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s9-R-CN2.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s9-R-CN3.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s9-R-CN4.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s9-B-bills.xml: 46 bytes
  STATUS <absent>
  BILLFIXED groups: 0
  groups for 'BRIDGE Walk-in BC': 0
== a-s9-B-payable.xml: 46 bytes
  STATUS <absent>
  BILLFIXED groups: 0
  groups for 'BRIDGE Walk-in BC': 0
== a-s9-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

## pb.py outputs, whole (the snapshot and report answers)

### s2
```
a-s2-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 18
  'BRIDGE Walk-in BC': no row
exit 0
```

### s4sa
```
a-s4sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-2102.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -2102.00
exit 0
```

### s4sb
```
a-s4sb-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-2102.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -2102.00
exit 0
```

### s4la
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['1742']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -2102.00
ANSWER: ROWS 2 (0 On Account)
exit 0
```

### s4lb
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['1742']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -2102.00
ANSWER: ROWS 2 (0 On Account)
exit 0
```

### s5sa
```
a-s5sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
exit 0
```

### s5sb
```
a-s5sb-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
exit 0
```

### s5la
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### s5lb
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### s6sa
```
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-525.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -525.50
exit 0
```

### s6sb
```
a-s6sb-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-525.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -525.50
exit 0
```

### s6la
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['1282']
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -525.50
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### s6lb
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['1282']
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -525.50
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### s7sa
```
a-s7sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '525.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 525.50
exit 0
```

### s7sb
```
a-s7sb-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '525.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 525.50
exit 0
```

### s7la
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['2906']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: 525.50
ANSWER: ROWS 2 (0 On Account)
exit 0
```

### s7lb
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['2906']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: 525.50
ANSWER: ROWS 2 (0 On Account)
exit 0
```

### s8sa
```
a-s8sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '1576.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 1576.50
exit 0
```

### s8sb
```
a-s8sb-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '1576.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 1576.50
exit 0
```

### s8la
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['3720']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0098' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: 1576.50
ANSWER: ROWS 3 (0 On Account)
exit 0
```

### s8lb
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['3720']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0098' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: 1576.50
ANSWER: ROWS 3 (0 On Account)
exit 0
```

### s9
```
a-s9-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 18
  'BRIDGE Walk-in BC': no row
exit 0
```

## pb.py --outcome, whole

### after s4
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26728']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26728']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['1742']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['1742']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -2102.00
ANSWER: ROWS 2 (0 On Account)
a-s4sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-2102.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -2102.00
OUTCOME: told (ROWS 2 (0 On Account), sum of BILLCL -2102.00 equals closing -2102.00)
exit 0
```

### after s5
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26728']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26728']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s5sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### after s6
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26724']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26724']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['1282']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['1282']
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -525.50
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-525.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -525.50
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -525.50 equals closing -525.50)
exit 0
```

### after s7
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26724']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26724']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['2906']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['2906']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: 525.50
ANSWER: ROWS 2 (0 On Account)
a-s7sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '525.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 525.50
OUTCOME: told (ROWS 2 (0 On Account), sum of BILLCL 525.50 equals closing 525.50)
exit 0
```

### after s8
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26724']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26724']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['3720']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['3720']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0098' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: 1576.50
ANSWER: ROWS 3 (0 On Account)
a-s8sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '1576.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 1576.50
OUTCOME: told (ROWS 3 (0 On Account), sum of BILLCL 1576.50 equals closing 1576.50)
exit 0
```

## Compares, checks and consoles, whole

### s3m: M against s1 (the before-write check)
```
== a-s3m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5m: M against s1 (the before-write check)
```
== a-s5m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5r: R-0099 against its last earlier answer (--diff)
```
== --diff a-s4r-R-0099.xml a-s5r-R-0099.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
```

### s5r: R-0101 against its last earlier answer (--diff)
```
== --diff a-s4r-R-0101.xml a-s5r-R-0101.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
```

### br.sh at s5: its console
```
S-snap-20260401-20260801.xml: 26728 bytes
a-s5sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
exit 0
S-snap-20260401-20260801.xml: 26728 bytes
a-s5sb-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
exit 0
L-lo-bc-20260801.xml: 896 bytes
printwindow True saved <LAB_DIR>/shots/s5la-after.png
2026-11-10T05:41:22.392Z
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
L-lo-bc-20260801.xml: 896 bytes
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
exit 0
B-bills.xml: 510 bytes
B-payable.xml: 46 bytes
M-marks.xml: 3396 bytes
R-CN1.xml: 95834 bytes
R-0099.xml: 95834 bytes
R-0101.xml: 95834 bytes
== a-s5r-B-bills.xml: 510 bytes
  STATUS <absent>
  BILLFIXED groups: 1
  groups for 'BRIDGE Walk-in BC': 1
   BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-1051.00'
== a-s5r-B-payable.xml: 46 bytes
  STATUS <absent>
  BILLFIXED groups: 0
  groups for 'BRIDGE Walk-in BC': 0
== a-s5r-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '71' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s5r-R-CN1.xml: 95834 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f'
   ALTERID '71'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0099'; BILLTYPE 'Agst Ref'; AMOUNT '1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '2'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s5r-R-0099.xml: 95834 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0099'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
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
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0099'; BILLTYPE 'New Ref'; AMOUNT '-1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '2'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s5r-R-0101.xml: 95834 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0101'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0101'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e'
   ALTERID '70'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0101'; BILLTYPE 'New Ref'; AMOUNT '-1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '3'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
exit 0
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26728']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26728']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['896']
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -1051.00
ANSWER: ROWS 1 (0 On Account)
a-s5sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-1051.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -1051.00
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -1051.00 equals closing -1051.00)
exit 0
```

### s6m: M against s1 (the before-write check)
```
== a-s6m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s6r: R-0101 against its last earlier answer (--diff)
```
== --diff a-s5r-R-0101.xml a-s6r-R-0101.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
```

### s6r: R-CN1 against its last earlier answer (--diff)
```
== --diff a-s5r-R-CN1.xml a-s6r-R-CN1.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
```

### br.sh at s6: its console
```
S-snap-20260401-20260801.xml: 26724 bytes
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-525.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -525.50
exit 0
S-snap-20260401-20260801.xml: 26724 bytes
a-s6sb-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-525.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -525.50
exit 0
L-lo-bc-20260801.xml: 1282 bytes
printwindow True saved <LAB_DIR>/shots/s6la-after.png
2026-11-10T05:42:32.281Z
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['1282']
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -525.50
ANSWER: ROWS 1 (0 On Account)
exit 0
L-lo-bc-20260801.xml: 1282 bytes
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['1282']
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -525.50
ANSWER: ROWS 1 (0 On Account)
exit 0
B-bills.xml: 508 bytes
B-payable.xml: 46 bytes
M-marks.xml: 3396 bytes
R-CN2.xml: 95830 bytes
R-CN1.xml: 95834 bytes
R-0101.xml: 95834 bytes
== a-s6r-B-bills.xml: 508 bytes
  STATUS <absent>
  BILLFIXED groups: 1
  groups for 'BRIDGE Walk-in BC': 1
   BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50'
== a-s6r-B-payable.xml: 46 bytes
  STATUS <absent>
  BILLFIXED groups: 0
  groups for 'BRIDGE Walk-in BC': 0
== a-s6r-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '72' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s6r-R-CN2.xml: 95830 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '2'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0101'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000030'
   ALTERID '72'
   VCHGSTSTATUSISINCLUDED 'No'
   VCHGSTSTATUSISUNCERTAIN 'Yes'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'No' | '525.50'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-300.20'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-200.20'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-12.51'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-12.51'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.08'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0101'; BILLTYPE 'Agst Ref'; AMOUNT '525.50'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '3'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s6r-R-CN1.xml: 95834 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f'
   ALTERID '71'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0099'; BILLTYPE 'Agst Ref'; AMOUNT '1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '2'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s6r-R-0101.xml: 95834 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0101'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0101'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e'
   ALTERID '70'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0101'; BILLTYPE 'New Ref'; AMOUNT '-1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '3'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
exit 0
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26724']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26724']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['1282']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['1282']
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: -525.50
ANSWER: ROWS 1 (0 On Account)
a-s6sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '-525.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads -525.50
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL -525.50 equals closing -525.50)
exit 0
```

### s7m: M against s1 (the before-write check)
```
== a-s7m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s7r: R-0099 against its last earlier answer (--diff)
```
== --diff a-s5r-R-0099.xml a-s7r-R-0099.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
```

### s7r: R-CN1 against its last earlier answer (--diff)
```
== --diff a-s6r-R-CN1.xml a-s7r-R-CN1.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
```

### br.sh at s7: its console
```
S-snap-20260401-20260801.xml: 26724 bytes
a-s7sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '525.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 525.50
exit 0
S-snap-20260401-20260801.xml: 26724 bytes
a-s7sb-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '525.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 525.50
exit 0
L-lo-bc-20260801.xml: 2906 bytes
printwindow True saved <LAB_DIR>/shots/s7la-after.png
2026-11-10T05:43:36.982Z
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['2906']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: 525.50
ANSWER: ROWS 2 (0 On Account)
exit 0
L-lo-bc-20260801.xml: 2906 bytes
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['2906']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: 525.50
ANSWER: ROWS 2 (0 On Account)
exit 0
B-bills.xml: 508 bytes
B-payable.xml: 508 bytes
M-marks.xml: 3398 bytes
R-CN3.xml: 95836 bytes
R-CN1.xml: 95836 bytes
R-0099.xml: 95836 bytes
== a-s7r-B-bills.xml: 508 bytes
  STATUS <absent>
  BILLFIXED groups: 1
  groups for 'BRIDGE Walk-in BC': 1
   BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50'
== a-s7r-B-payable.xml: 508 bytes
  STATUS <absent>
  BILLFIXED groups: 1
  groups for 'BRIDGE Walk-in BC': 1
   BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00'
== a-s7r-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '73' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s7r-R-CN3.xml: 95836 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '3'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000031'
   ALTERID '73'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0099'; BILLTYPE 'Agst Ref'; AMOUNT '1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '2'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s7r-R-CN1.xml: 95836 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f'
   ALTERID '71'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0099'; BILLTYPE 'Agst Ref'; AMOUNT '1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '2'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s7r-R-0099.xml: 95836 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0099'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
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
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0099'; BILLTYPE 'New Ref'; AMOUNT '-1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '2'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
exit 0
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26724']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26724']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['2906']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['2906']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: 525.50
ANSWER: ROWS 2 (0 On Account)
a-s7sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '525.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 525.50
OUTCOME: told (ROWS 2 (0 On Account), sum of BILLCL 525.50 equals closing 525.50)
exit 0
```

### s8m: M against s1 (the before-write check)
```
== a-s8m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'73'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s8r: R-0099 against its last earlier answer (--diff)
```
== --diff a-s7r-R-0099.xml a-s8r-R-0099.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
```

### s8r: R-CN1 against its last earlier answer (--diff)
```
== --diff a-s7r-R-CN1.xml a-s8r-R-CN1.xml
   lists compared: 1
   legs: 6 -> 6
exit 0
```

### br.sh at s8: its console
```
S-snap-20260401-20260801.xml: 26724 bytes
a-s8sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '1576.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 1576.50
exit 0
S-snap-20260401-20260801.xml: 26724 bytes
a-s8sb-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '1576.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 1576.50
exit 0
L-lo-bc-20260801.xml: 3720 bytes
printwindow True saved <LAB_DIR>/shots/s8la-after.png
2026-11-10T05:44:30.936Z
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['3720']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0098' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: 1576.50
ANSWER: ROWS 3 (0 On Account)
exit 0
L-lo-bc-20260801.xml: 3720 bytes
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['3720']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0098' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: 1576.50
ANSWER: ROWS 3 (0 On Account)
exit 0
B-bills.xml: 508 bytes
B-payable.xml: 966 bytes
M-marks.xml: 3398 bytes
R-CN4.xml: 95836 bytes
R-CN1.xml: 95836 bytes
R-0099.xml: 95836 bytes
== a-s8r-B-bills.xml: 508 bytes
  STATUS <absent>
  BILLFIXED groups: 1
  groups for 'BRIDGE Walk-in BC': 1
   BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50'
== a-s8r-B-payable.xml: 966 bytes
  STATUS <absent>
  BILLFIXED groups: 2
  groups for 'BRIDGE Walk-in BC': 2
   BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00'
   BILLREF 'BP/26-27/0098' BILLDATE '1-Aug-26' BILLCL '1051.00'
== a-s8r-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '74' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s8r-R-CN4.xml: 95836 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '4'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000032'
   ALTERID '74'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0098'; BILLTYPE 'New Ref'; AMOUNT '1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '4'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s8r-R-CN1.xml: 95836 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
   PARTYGSTIN ''
   STATENAME 'Rajasthan'
   PLACEOFSUPPLY 'Rajasthan'
   GSTREGISTRATIONTYPE 'Unregistered/Consumer'
   ISINVOICE 'Yes'
   ISCANCELLED 'No'
   ISOPTIONAL 'No'
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f'
   ALTERID '71'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0099'; BILLTYPE 'Agst Ref'; AMOUNT '1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '2'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s8r-R-0099.xml: 95836 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0099'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walk-in BC'
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
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in BC' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   bills on 'BRIDGE Walk-in BC': 1 with NAME, 0 whitespace-only, 1 lists
      NAME 'BP/26-27/0099'; BILLTYPE 'New Ref'; AMOUNT '-1051.00'; BILLDATE '20260801'; BILLCREATIONDATE '20260801'; BILLID '2'
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
exit 0
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26724']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26724']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['3720']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['3720']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0098' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: 1576.50
ANSWER: ROWS 3 (0 On Account)
a-s8sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walk-in BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '1576.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 1576.50
OUTCOME: told (ROWS 3 (0 On Account), sum of BILLCL 1576.50 equals closing 1576.50)
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
$ each answer of s9 against the same request's answer at s2, by sha256
N-0099.xml: s2 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974, s9 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974: equal
N-0101.xml: s2 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974, s9 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974: equal
R-0099.xml: s2 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974, s9 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974: equal
R-0101.xml: s2 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974, s9 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974: equal
R-CN1.xml: s2 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974, s9 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974: equal
R-CN2.xml: s2 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974, s9 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974: equal
R-CN3.xml: s2 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974, s9 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974: equal
R-CN4.xml: s2 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974, s9 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974: equal
B-bills.xml: s2 8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3, s9 8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3: equal
B-payable.xml: s2 8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3, s9 8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3: equal
S-snap-20260401-20260801.xml: s2 ffe23742ab0793221f0136c5e119bde7d2e8fd2181f01e0f31520f61608eab69, s9 ffe23742ab0793221f0136c5e119bde7d2e8fd2181f01e0f31520f61608eab69: equal
M-marks.xml: s2 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6, s9 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6: equal
```

### step 1: the book's data folder and the excluded GUID
```
BRIDGE PILOT LAB: folder <N>, GUID hits 2, excluded GUID hits 0
folders named so: 1
```

### DESC/CMPINFO count lines of every answer
```
$ the DESC/CMPINFO count lines of every read answer that has them (LAB-COMMON C4: not rows); the names with a count other than 0, then the rest as "others 0"
a-s1-M-marks.xml: all 0 (names in all: 35)
a-s2-N-0099.xml: all 0 (names in all: 35)
a-s2-N-0101.xml: all 0 (names in all: 35)
a-s2-R-0099.xml: all 0 (names in all: 35)
a-s2-R-0101.xml: all 0 (names in all: 35)
a-s2-R-CN1.xml: all 0 (names in all: 35)
a-s2-R-CN2.xml: all 0 (names in all: 35)
a-s2-R-CN3.xml: all 0 (names in all: 35)
a-s2-R-CN4.xml: all 0 (names in all: 35)
a-s2-S-snap-20260401-20260801.xml: all 0 (names in all: 35)
a-s2-M-marks.xml: all 0 (names in all: 35)
a-s3m-M-marks.xml: all 0 (names in all: 35)
a-s4sa-S-snap-20260401-20260801.xml: LEDGER 23; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 4; others 0 (names in all: 35)
a-s4sb-S-snap-20260401-20260801.xml: LEDGER 23; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 4; others 0 (names in all: 35)
a-s4r-M-marks.xml: LEDGER 23; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 4; others 0 (names in all: 35)
a-s4r-N-0099.xml: LEDGER 23; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 4; others 0 (names in all: 35)
a-s4r-N-0101.xml: LEDGER 23; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 4; others 0 (names in all: 35)
a-s4r-R-0099.xml: LEDGER 23; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 4; others 0 (names in all: 35)
a-s4r-R-0101.xml: LEDGER 23; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 4; others 0 (names in all: 35)
a-s5m-M-marks.xml: LEDGER 23; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 4; others 0 (names in all: 35)
a-s5sa-S-snap-20260401-20260801.xml: LEDGER 34; CURRENCY 3; TAXUNIT 3; VOUCHERNUMBERSERIES 3; VOUCHER 6; others 0 (names in all: 35)
a-s5sb-S-snap-20260401-20260801.xml: LEDGER 34; CURRENCY 3; TAXUNIT 3; VOUCHERNUMBERSERIES 3; VOUCHER 6; others 0 (names in all: 35)
a-s5r-M-marks.xml: LEDGER 34; CURRENCY 3; TAXUNIT 3; VOUCHERNUMBERSERIES 3; VOUCHER 6; others 0 (names in all: 35)
a-s5r-R-CN1.xml: LEDGER 34; CURRENCY 3; TAXUNIT 3; VOUCHERNUMBERSERIES 3; VOUCHER 6; others 0 (names in all: 35)
a-s5r-R-0099.xml: LEDGER 34; CURRENCY 3; TAXUNIT 3; VOUCHERNUMBERSERIES 3; VOUCHER 6; others 0 (names in all: 35)
a-s5r-R-0101.xml: LEDGER 34; CURRENCY 3; TAXUNIT 3; VOUCHERNUMBERSERIES 3; VOUCHER 6; others 0 (names in all: 35)
a-s6m-M-marks.xml: LEDGER 34; CURRENCY 3; TAXUNIT 3; VOUCHERNUMBERSERIES 3; VOUCHER 6; others 0 (names in all: 35)
a-s6sa-S-snap-20260401-20260801.xml: LEDGER 45; CURRENCY 4; TAXUNIT 4; VOUCHERNUMBERSERIES 4; VOUCHER 8; others 0 (names in all: 35)
a-s6sb-S-snap-20260401-20260801.xml: LEDGER 45; CURRENCY 4; TAXUNIT 4; VOUCHERNUMBERSERIES 4; VOUCHER 8; others 0 (names in all: 35)
a-s6r-M-marks.xml: LEDGER 45; CURRENCY 4; TAXUNIT 4; VOUCHERNUMBERSERIES 4; VOUCHER 8; others 0 (names in all: 35)
a-s6r-R-CN2.xml: LEDGER 45; CURRENCY 4; TAXUNIT 4; VOUCHERNUMBERSERIES 4; VOUCHER 8; others 0 (names in all: 35)
a-s6r-R-CN1.xml: LEDGER 45; CURRENCY 4; TAXUNIT 4; VOUCHERNUMBERSERIES 4; VOUCHER 8; others 0 (names in all: 35)
a-s6r-R-0101.xml: LEDGER 45; CURRENCY 4; TAXUNIT 4; VOUCHERNUMBERSERIES 4; VOUCHER 8; others 0 (names in all: 35)
a-s7m-M-marks.xml: LEDGER 45; CURRENCY 4; TAXUNIT 4; VOUCHERNUMBERSERIES 4; VOUCHER 8; others 0 (names in all: 35)
a-s7sa-S-snap-20260401-20260801.xml: LEDGER 56; CURRENCY 5; TAXUNIT 5; VOUCHERNUMBERSERIES 5; VOUCHER 10; others 0 (names in all: 35)
a-s7sb-S-snap-20260401-20260801.xml: LEDGER 56; CURRENCY 5; TAXUNIT 5; VOUCHERNUMBERSERIES 5; VOUCHER 10; others 0 (names in all: 35)
a-s7r-M-marks.xml: LEDGER 56; CURRENCY 5; TAXUNIT 5; VOUCHERNUMBERSERIES 5; VOUCHER 10; others 0 (names in all: 35)
a-s7r-R-CN3.xml: LEDGER 56; CURRENCY 5; TAXUNIT 5; VOUCHERNUMBERSERIES 5; VOUCHER 10; others 0 (names in all: 35)
a-s7r-R-CN1.xml: LEDGER 56; CURRENCY 5; TAXUNIT 5; VOUCHERNUMBERSERIES 5; VOUCHER 10; others 0 (names in all: 35)
a-s7r-R-0099.xml: LEDGER 56; CURRENCY 5; TAXUNIT 5; VOUCHERNUMBERSERIES 5; VOUCHER 10; others 0 (names in all: 35)
a-s8m-M-marks.xml: LEDGER 56; CURRENCY 5; TAXUNIT 5; VOUCHERNUMBERSERIES 5; VOUCHER 10; others 0 (names in all: 35)
a-s8sa-S-snap-20260401-20260801.xml: LEDGER 67; CURRENCY 6; TAXUNIT 6; VOUCHERNUMBERSERIES 6; VOUCHER 12; others 0 (names in all: 35)
a-s8sb-S-snap-20260401-20260801.xml: LEDGER 67; CURRENCY 6; TAXUNIT 6; VOUCHERNUMBERSERIES 6; VOUCHER 12; others 0 (names in all: 35)
a-s8r-M-marks.xml: LEDGER 67; CURRENCY 6; TAXUNIT 6; VOUCHERNUMBERSERIES 6; VOUCHER 12; others 0 (names in all: 35)
a-s8r-R-CN4.xml: LEDGER 67; CURRENCY 6; TAXUNIT 6; VOUCHERNUMBERSERIES 6; VOUCHER 12; others 0 (names in all: 35)
a-s8r-R-CN1.xml: LEDGER 67; CURRENCY 6; TAXUNIT 6; VOUCHERNUMBERSERIES 6; VOUCHER 12; others 0 (names in all: 35)
a-s8r-R-0099.xml: LEDGER 67; CURRENCY 6; TAXUNIT 6; VOUCHERNUMBERSERIES 6; VOUCHER 12; others 0 (names in all: 35)
a-s9-N-0099.xml: all 0 (names in all: 35)
a-s9-N-0101.xml: all 0 (names in all: 35)
a-s9-R-0099.xml: all 0 (names in all: 35)
a-s9-R-0101.xml: all 0 (names in all: 35)
a-s9-R-CN1.xml: all 0 (names in all: 35)
a-s9-R-CN2.xml: all 0 (names in all: 35)
a-s9-R-CN3.xml: all 0 (names in all: 35)
a-s9-R-CN4.xml: all 0 (names in all: 35)
a-s9-S-snap-20260401-20260801.xml: all 0 (names in all: 35)
a-s9-M-marks.xml: all 0 (names in all: 35)
```

### sha256 of every answer and headers file
```
$ sha256sum and size of every answer and its headers file, in send order
a-s1-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s1-M-marks.xml.headers 134 B a8c6fe9f6545175fb5e08b9ee53ba543601c0436c80c231f7b009d73e5c8ccf0
a-s2-N-0099.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-N-0099.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s2-N-0101.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-N-0101.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s2-R-0099.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-R-0099.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s2-R-0101.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-R-0101.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s2-R-CN1.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-R-CN1.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s2-R-CN2.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-R-CN2.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s2-R-CN3.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-R-CN3.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s2-R-CN4.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-R-CN4.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s2-B-bills.xml 46 B 8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3
a-s2-B-bills.xml.headers 132 B 64cfab401cbb196abaed18a52645f02e91cc2cc3dd24eda9de98476a77e0f2da
a-s2-B-payable.xml 46 B 8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3
a-s2-B-payable.xml.headers 132 B 64cfab401cbb196abaed18a52645f02e91cc2cc3dd24eda9de98476a77e0f2da
a-s2-S-snap-20260401-20260801.xml 25480 B ffe23742ab0793221f0136c5e119bde7d2e8fd2181f01e0f31520f61608eab69
a-s2-S-snap-20260401-20260801.xml.headers 135 B 07ac9dc906b1544708fc63eca58a56aacb3bb969a35fde4f27a334add3ad1c64
a-s2-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s2-M-marks.xml.headers 134 B a8c6fe9f6545175fb5e08b9ee53ba543601c0436c80c231f7b009d73e5c8ccf0
a-s3m-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s3m-M-marks.xml.headers 134 B a8c6fe9f6545175fb5e08b9ee53ba543601c0436c80c231f7b009d73e5c8ccf0
a-s3-L-ledger.xml 538 B 1b72152a8fd20ccb9366277864bf32357b8891e536cde2a54992adaa60a97010
a-s3-L-ledger.xml.headers 133 B 19b28d1b18183bea529f520e0ce9251ee2655e6b58302a8b47562695a42a91b7
a-s4-I-0099.xml 540 B fdcd8435c6fcf95ab624cbe2cff2c887078df500626f7c9c37cfa834a1cdb6ea
a-s4-I-0099.xml.headers 133 B 2a0eba722d654c2b2fa59d71608c67e3849d25c56207357fa06b0239977a4670
a-s4b-I-0101.xml 540 B 32e7a29b201c773547aa47bc95bd3eda8af83aeb6e35090303b9b27bcd34eec5
a-s4b-I-0101.xml.headers 133 B 2a0eba722d654c2b2fa59d71608c67e3849d25c56207357fa06b0239977a4670
a-s4sa-S-snap-20260401-20260801.xml 26728 B 72b4f0930061212a6fd8e76f288dbfc2b228353ca7782dbaf4dc5935deb7931e
a-s4sa-S-snap-20260401-20260801.xml.headers 135 B 7fda2ce7b26e497658a710780e202e2ab3ae635899791eee31ec85ac83f809ad
a-s4sb-S-snap-20260401-20260801.xml 26728 B 72b4f0930061212a6fd8e76f288dbfc2b228353ca7782dbaf4dc5935deb7931e
a-s4sb-S-snap-20260401-20260801.xml.headers 135 B 7fda2ce7b26e497658a710780e202e2ab3ae635899791eee31ec85ac83f809ad
a-s4la-L-lo-bc-20260801.xml 1742 B 5b05fb2490517a603ec33c1acb6c7c88cb694366bcd1fe98b172870fa7a2294a
a-s4la-L-lo-bc-20260801.xml.headers 134 B d75bc20a8ffd4bc40f3d1cfd89ba7fa16dce789874a9f8220bba5ac7dc113fba
a-s4lb-L-lo-bc-20260801.xml 1742 B 5b05fb2490517a603ec33c1acb6c7c88cb694366bcd1fe98b172870fa7a2294a
a-s4lb-L-lo-bc-20260801.xml.headers 134 B d75bc20a8ffd4bc40f3d1cfd89ba7fa16dce789874a9f8220bba5ac7dc113fba
a-s4r-B-bills.xml 970 B b97a1a0717db0986c58bfa56164a705f74fe83e9f78dbf330994fff8679f12e3
a-s4r-B-bills.xml.headers 133 B 3656bbb0e44edc2be2fc48530f86c88e22f3e0a73f43c1751dc55cceb1db0602
a-s4r-B-payable.xml 46 B 8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3
a-s4r-B-payable.xml.headers 132 B 64cfab401cbb196abaed18a52645f02e91cc2cc3dd24eda9de98476a77e0f2da
a-s4r-M-marks.xml 3396 B d55cf90ef81a5c355be9e694f8fae317da2e1701fa95feb6f4cc446cb4a7553e
a-s4r-M-marks.xml.headers 134 B db9b137db28569a10d72e1f9edb1f97fa4dc3c0f0c092ea2d8b3e1ba4cae016c
a-s4r-N-0099.xml 5608 B c94a8fe356b9f36a398518d3514d7b2a10485d40cd7fbc1583d4db114d31aeff
a-s4r-N-0099.xml.headers 134 B b34f5abc1d2a71b6d9626e6226de47851e70eee53bf98ffd9bd3013c08aa7e20
a-s4r-N-0101.xml 5608 B 3b49e2d4241b99fe7e20f6f83a2aeea9ae96db8ef6de6f1dc745edfe68f56138
a-s4r-N-0101.xml.headers 134 B b34f5abc1d2a71b6d9626e6226de47851e70eee53bf98ffd9bd3013c08aa7e20
a-s4r-R-0099.xml 95834 B 12f34e988ae0f75ddd957b55872a60fba98e3d7a018fd4f12439d773554db974
a-s4r-R-0099.xml.headers 135 B c98cf4a264d1f91d3064055e623338cda616c9f421109176ce8bd7fea484d34f
a-s4r-R-0101.xml 95834 B 77e52c6adeb5ca44f5a34d797307dec9a8cfadda4d22e64efa2439f7c89607c2
a-s4r-R-0101.xml.headers 135 B c98cf4a264d1f91d3064055e623338cda616c9f421109176ce8bd7fea484d34f
a-s5m-M-marks.xml 3396 B d55cf90ef81a5c355be9e694f8fae317da2e1701fa95feb6f4cc446cb4a7553e
a-s5m-M-marks.xml.headers 134 B db9b137db28569a10d72e1f9edb1f97fa4dc3c0f0c092ea2d8b3e1ba4cae016c
a-s5-I-CN-1.xml 540 B f8384a874e1d8a6be92c2969f94e2887fb700768f6e111b6b3f649cd13ffdc7e
a-s5-I-CN-1.xml.headers 133 B 2a0eba722d654c2b2fa59d71608c67e3849d25c56207357fa06b0239977a4670
a-s5sa-S-snap-20260401-20260801.xml 26728 B d92e7f86a05492aca342d659cce429d933dc63320d7ea5c12c80f333f11f887e
a-s5sa-S-snap-20260401-20260801.xml.headers 135 B 7fda2ce7b26e497658a710780e202e2ab3ae635899791eee31ec85ac83f809ad
a-s5sb-S-snap-20260401-20260801.xml 26728 B d92e7f86a05492aca342d659cce429d933dc63320d7ea5c12c80f333f11f887e
a-s5sb-S-snap-20260401-20260801.xml.headers 135 B 7fda2ce7b26e497658a710780e202e2ab3ae635899791eee31ec85ac83f809ad
a-s5la-L-lo-bc-20260801.xml 896 B 8746152cec3d3b8305159ebff5b4aef6552f3949e84e29a69de675f14c30ab46
a-s5la-L-lo-bc-20260801.xml.headers 133 B 64a6e548b8c46f0d9bb97b2b99d44aaf99a3122b9109bca9abadf55fd2d05080
a-s5lb-L-lo-bc-20260801.xml 896 B 8746152cec3d3b8305159ebff5b4aef6552f3949e84e29a69de675f14c30ab46
a-s5lb-L-lo-bc-20260801.xml.headers 133 B 64a6e548b8c46f0d9bb97b2b99d44aaf99a3122b9109bca9abadf55fd2d05080
a-s5r-B-bills.xml 510 B 6a3efd62ca11ab03d111aeee76f134988bdb1f4670badf4aa51a3a3add3b63fb
a-s5r-B-bills.xml.headers 133 B fa7fb12b5ab77578a09a481d44d0eaeb7caa759aa8018295c27a663c227d9329
a-s5r-B-payable.xml 46 B 8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3
a-s5r-B-payable.xml.headers 132 B 64cfab401cbb196abaed18a52645f02e91cc2cc3dd24eda9de98476a77e0f2da
a-s5r-M-marks.xml 3396 B 4ac186ec14f33c8066791cc711c6b86445781db3758c5abc3d982280fa4b5b7f
a-s5r-M-marks.xml.headers 134 B db9b137db28569a10d72e1f9edb1f97fa4dc3c0f0c092ea2d8b3e1ba4cae016c
a-s5r-R-CN1.xml 95834 B 9b3c4aa1014a7982915966acd7120ff34c2be0b529657865082ef7cbe3be1f3c
a-s5r-R-CN1.xml.headers 135 B c98cf4a264d1f91d3064055e623338cda616c9f421109176ce8bd7fea484d34f
a-s5r-R-0099.xml 95834 B 23568fb72aae4d27e24230b91561588e3a39222ff2f516fdf2213b6dffb8fd43
a-s5r-R-0099.xml.headers 135 B c98cf4a264d1f91d3064055e623338cda616c9f421109176ce8bd7fea484d34f
a-s5r-R-0101.xml 95834 B a2ec8f2c500ee0277892848e80af3c7eca7020cb3e59f8acd9745a3b878972f7
a-s5r-R-0101.xml.headers 135 B c98cf4a264d1f91d3064055e623338cda616c9f421109176ce8bd7fea484d34f
a-s6m-M-marks.xml 3396 B 4ac186ec14f33c8066791cc711c6b86445781db3758c5abc3d982280fa4b5b7f
a-s6m-M-marks.xml.headers 134 B db9b137db28569a10d72e1f9edb1f97fa4dc3c0f0c092ea2d8b3e1ba4cae016c
a-s6-I-CN-P.xml 540 B e6540f6534095f5608f3c59f802c1f40a3ae13529a7e9041847b460f03c32fec
a-s6-I-CN-P.xml.headers 133 B 2a0eba722d654c2b2fa59d71608c67e3849d25c56207357fa06b0239977a4670
a-s6sa-S-snap-20260401-20260801.xml 26724 B 04f6c60ea49e3d3fa40185ff46b1e3fb190862bbefef1b7597597d065ddbcd39
a-s6sa-S-snap-20260401-20260801.xml.headers 135 B b261143a215b3a643818a8b82f0479908431f4b266f70daf95e75c9963afa574
a-s6sb-S-snap-20260401-20260801.xml 26724 B 04f6c60ea49e3d3fa40185ff46b1e3fb190862bbefef1b7597597d065ddbcd39
a-s6sb-S-snap-20260401-20260801.xml.headers 135 B b261143a215b3a643818a8b82f0479908431f4b266f70daf95e75c9963afa574
a-s6la-L-lo-bc-20260801.xml 1282 B a566f4cebec8b64221fa889738afefbba6ded728e56f7ae108ad4f72bba324c5
a-s6la-L-lo-bc-20260801.xml.headers 134 B 28df636807d62d9b9d434f1e9d033d15e5ac297681c97b96396ec575aeef0e23
a-s6lb-L-lo-bc-20260801.xml 1282 B a566f4cebec8b64221fa889738afefbba6ded728e56f7ae108ad4f72bba324c5
a-s6lb-L-lo-bc-20260801.xml.headers 134 B 28df636807d62d9b9d434f1e9d033d15e5ac297681c97b96396ec575aeef0e23
a-s6r-B-bills.xml 508 B 5503268c3303f4b75cdc949d3a68768e141de143eadaf98a42b4469c4e7fa823
a-s6r-B-bills.xml.headers 133 B c2b735228562b5e068bf3a34eb76dd077690b1373141e74e9f63f4c45d409ab7
a-s6r-B-payable.xml 46 B 8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3
a-s6r-B-payable.xml.headers 132 B 64cfab401cbb196abaed18a52645f02e91cc2cc3dd24eda9de98476a77e0f2da
a-s6r-M-marks.xml 3396 B 419f6b56d7e3f2add5ccb53dede6f88457ffc8a20399d3d60a4786a0d48e062e
a-s6r-M-marks.xml.headers 134 B db9b137db28569a10d72e1f9edb1f97fa4dc3c0f0c092ea2d8b3e1ba4cae016c
a-s6r-R-CN2.xml 95830 B eb470e67de44a4aa5afd63fa5bbf3e236c2c150ce7ffb9bf66f7a554259a2ae0
a-s6r-R-CN2.xml.headers 135 B a726cd890d17a6db9c6ae30b4bcbaeebab557bddbfee200a3b4e7169835aeaab
a-s6r-R-CN1.xml 95834 B 0310f5df5972ebdd71f8048c6bba9fac191d4ab49a2bbbe42751d6cd27203776
a-s6r-R-CN1.xml.headers 135 B c98cf4a264d1f91d3064055e623338cda616c9f421109176ce8bd7fea484d34f
a-s6r-R-0101.xml 95834 B 6b044785bf191d462b5c15f6131cc238c49e80a90289aac260b00ec992625b05
a-s6r-R-0101.xml.headers 135 B c98cf4a264d1f91d3064055e623338cda616c9f421109176ce8bd7fea484d34f
a-s7m-M-marks.xml 3396 B 419f6b56d7e3f2add5ccb53dede6f88457ffc8a20399d3d60a4786a0d48e062e
a-s7m-M-marks.xml.headers 134 B db9b137db28569a10d72e1f9edb1f97fa4dc3c0f0c092ea2d8b3e1ba4cae016c
a-s7-I-CN-2.xml 540 B 93e331323bb1235565ab31f2ddbbaf7afaa752ce6770d0ae7ae145d7b3244234
a-s7-I-CN-2.xml.headers 133 B 2a0eba722d654c2b2fa59d71608c67e3849d25c56207357fa06b0239977a4670
a-s7sa-S-snap-20260401-20260801.xml 26724 B 28fb291447b74b7f439f8064c34eefd5b252ab0537f84a51e4f30318644f2f4d
a-s7sa-S-snap-20260401-20260801.xml.headers 135 B b261143a215b3a643818a8b82f0479908431f4b266f70daf95e75c9963afa574
a-s7sb-S-snap-20260401-20260801.xml 26724 B 28fb291447b74b7f439f8064c34eefd5b252ab0537f84a51e4f30318644f2f4d
a-s7sb-S-snap-20260401-20260801.xml.headers 135 B b261143a215b3a643818a8b82f0479908431f4b266f70daf95e75c9963afa574
a-s7la-L-lo-bc-20260801.xml 2906 B fd3badc8b8cd5523d47d283c470efbfd0fffab6b36e30f7d05806153bdfc157b
a-s7la-L-lo-bc-20260801.xml.headers 134 B b8bd0d85d02618b26e601bf98d59ece3f8d2f9fc675a96b4bc95c58d48cdbeab
a-s7lb-L-lo-bc-20260801.xml 2906 B fd3badc8b8cd5523d47d283c470efbfd0fffab6b36e30f7d05806153bdfc157b
a-s7lb-L-lo-bc-20260801.xml.headers 134 B b8bd0d85d02618b26e601bf98d59ece3f8d2f9fc675a96b4bc95c58d48cdbeab
a-s7r-B-bills.xml 508 B 5503268c3303f4b75cdc949d3a68768e141de143eadaf98a42b4469c4e7fa823
a-s7r-B-bills.xml.headers 133 B c2b735228562b5e068bf3a34eb76dd077690b1373141e74e9f63f4c45d409ab7
a-s7r-B-payable.xml 508 B 6547fc539003044e49723d3c941861e59031eedab6befd057d85204cb4a58f0d
a-s7r-B-payable.xml.headers 133 B c2b735228562b5e068bf3a34eb76dd077690b1373141e74e9f63f4c45d409ab7
a-s7r-M-marks.xml 3398 B 854002ea209a04d49b046a679264a052652a8f3b837c36c926ef7b7e0b1e84b1
a-s7r-M-marks.xml.headers 134 B 4d2c776864af2995685156531c0e1bd6dfa88116418863514e54213f2a5ee6a0
a-s7r-R-CN3.xml 95836 B 547e5dc8e82bee41b85142b00d4e66a0ea070732d1cdc6d2a498a1a6d8bdd8c5
a-s7r-R-CN3.xml.headers 135 B 32296da2754d444fa7009cd9d8575a8c29f1899a51f49549985cd00c9d1725ea
a-s7r-R-CN1.xml 95836 B 867d57fbf9b36d2e076b5ed64c3da7b6fcc726230a6daf810837d9851ffcb931
a-s7r-R-CN1.xml.headers 135 B 32296da2754d444fa7009cd9d8575a8c29f1899a51f49549985cd00c9d1725ea
a-s7r-R-0099.xml 95836 B ac1d60a8eacdecfac8a8daadde13b5f649ba7c97c25bda37cec53ee27f497c5d
a-s7r-R-0099.xml.headers 135 B 32296da2754d444fa7009cd9d8575a8c29f1899a51f49549985cd00c9d1725ea
a-s8m-M-marks.xml 3398 B 854002ea209a04d49b046a679264a052652a8f3b837c36c926ef7b7e0b1e84b1
a-s8m-M-marks.xml.headers 134 B 4d2c776864af2995685156531c0e1bd6dfa88116418863514e54213f2a5ee6a0
a-s8-I-CN-3.xml 540 B 200c17e120c3ff582d28c9fce94b127ed22aaa4e5d10bbdd1d18e97a767f40c3
a-s8-I-CN-3.xml.headers 133 B 2a0eba722d654c2b2fa59d71608c67e3849d25c56207357fa06b0239977a4670
a-s8sa-S-snap-20260401-20260801.xml 26724 B a20c20f0e8bfc8e0180b844f29f73fcc01659b5b12850875fe0ddcea16b3a07e
a-s8sa-S-snap-20260401-20260801.xml.headers 135 B b261143a215b3a643818a8b82f0479908431f4b266f70daf95e75c9963afa574
a-s8sb-S-snap-20260401-20260801.xml 26724 B a20c20f0e8bfc8e0180b844f29f73fcc01659b5b12850875fe0ddcea16b3a07e
a-s8sb-S-snap-20260401-20260801.xml.headers 135 B b261143a215b3a643818a8b82f0479908431f4b266f70daf95e75c9963afa574
a-s8la-L-lo-bc-20260801.xml 3720 B 113fe494a6034279763f1e2d5fc14dd0e1a31465db206f6f4963993ea8938a13
a-s8la-L-lo-bc-20260801.xml.headers 134 B fc1252100299388875c14f55399a5c7ef8d50c25cd9dc253fdff0e4ccf6121e8
a-s8lb-L-lo-bc-20260801.xml 3720 B 113fe494a6034279763f1e2d5fc14dd0e1a31465db206f6f4963993ea8938a13
a-s8lb-L-lo-bc-20260801.xml.headers 134 B fc1252100299388875c14f55399a5c7ef8d50c25cd9dc253fdff0e4ccf6121e8
a-s8r-B-bills.xml 508 B 5503268c3303f4b75cdc949d3a68768e141de143eadaf98a42b4469c4e7fa823
a-s8r-B-bills.xml.headers 133 B c2b735228562b5e068bf3a34eb76dd077690b1373141e74e9f63f4c45d409ab7
a-s8r-B-payable.xml 966 B 1e6171e57aafef610bc0f193be89b80b7eb7a2316d38a5c07c15f282123b1bf3
a-s8r-B-payable.xml.headers 133 B e43a33e3b30c01f4d2e62279e9f6dd39feb4f665c85da915f0581429133e729f
a-s8r-M-marks.xml 3398 B 24c915d10371ba90319bb238ece00778da068326191e81b4c04d6ac59c5379a6
a-s8r-M-marks.xml.headers 134 B 4d2c776864af2995685156531c0e1bd6dfa88116418863514e54213f2a5ee6a0
a-s8r-R-CN4.xml 95836 B 8704315d4ea8dadc4e412cd1cbc320a0b43d82e792fdab6944dae3731dbb42bb
a-s8r-R-CN4.xml.headers 135 B 32296da2754d444fa7009cd9d8575a8c29f1899a51f49549985cd00c9d1725ea
a-s8r-R-CN1.xml 95836 B 6e5de6d41f4d687fed631808c6a8369f4300504e9d8fcd7fef13d2994bd6e26e
a-s8r-R-CN1.xml.headers 135 B 32296da2754d444fa7009cd9d8575a8c29f1899a51f49549985cd00c9d1725ea
a-s8r-R-0099.xml 95836 B 70844f32ad54d3eaac4042d701c12ae6c730f70bf3289c3c146e82468557bf5c
a-s8r-R-0099.xml.headers 135 B 32296da2754d444fa7009cd9d8575a8c29f1899a51f49549985cd00c9d1725ea
a-s9-N-0099.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s9-N-0099.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s9-N-0101.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s9-N-0101.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s9-R-0099.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s9-R-0099.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s9-R-0101.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s9-R-0101.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s9-R-CN1.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s9-R-CN1.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s9-R-CN2.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s9-R-CN2.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s9-R-CN3.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s9-R-CN3.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s9-R-CN4.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s9-R-CN4.xml.headers 134 B d8476b58186ccb6d4894910d74b76776d7ab0cd2d05a93c789aa5bc7a0957bcf
a-s9-B-bills.xml 46 B 8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3
a-s9-B-bills.xml.headers 132 B 64cfab401cbb196abaed18a52645f02e91cc2cc3dd24eda9de98476a77e0f2da
a-s9-B-payable.xml 46 B 8d37111f1de57f9c4d5ea3e984d10db165c5a8b28a0c0ec6b2688ffbc61d5ad3
a-s9-B-payable.xml.headers 132 B 64cfab401cbb196abaed18a52645f02e91cc2cc3dd24eda9de98476a77e0f2da
a-s9-S-snap-20260401-20260801.xml 25480 B ffe23742ab0793221f0136c5e119bde7d2e8fd2181f01e0f31520f61608eab69
a-s9-S-snap-20260401-20260801.xml.headers 135 B 07ac9dc906b1544708fc63eca58a56aacb3bb969a35fde4f27a334add3ad1c64
a-s9-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s9-M-marks.xml.headers 134 B a8c6fe9f6545175fb5e08b9ee53ba543601c0436c80c231f7b009d73e5c8ccf0
```

### each send's start and answer, UTC
```
2026-11-10T05:36:47.929Z s1 M-marks.xml (hashes equal to the list)
2026-11-10T05:36:49.109Z s1 answered
2026-11-10T05:38:52.478Z s2 N-0099.xml N-0101.xml R-0099.xml R-0101.xml R-CN1.xml R-CN2.xml R-CN3.xml R-CN4.xml B-bills.xml B-payable.xml S-snap-20260401-20260801.xml M-marks.xml (hashes equal to the list)
2026-11-10T05:38:54.862Z s2 answered
2026-11-10T05:39:15.565Z s3m M-marks.xml (hashes equal to the list)
2026-11-10T05:39:16.344Z s3m answered
2026-11-10T05:39:29.795Z s3 L-ledger.xml (hashes equal to the list)
2026-11-10T05:39:34.452Z s3 answered
2026-11-10T05:39:35.798Z s4 I-0099.xml (hashes equal to the list)
2026-11-10T05:39:42.302Z s4 answered
2026-11-10T05:39:54.006Z s4b I-0101.xml (hashes equal to the list)
2026-11-10T05:39:59.176Z s4b answered
2026-11-10T05:40:11.382Z s4sa S-snap-20260401-20260801.xml (hashes equal to the list)
2026-11-10T05:40:11.908Z s4sa answered
2026-11-10T05:40:13.033Z s4sb S-snap-20260401-20260801.xml (hashes equal to the list)
2026-11-10T05:40:13.757Z s4sb answered
2026-11-10T05:40:14.915Z s4la L-lo-bc-20260801.xml (hashes equal to the list)
2026-11-10T05:40:19.276Z s4la answered
2026-11-10T05:40:22.594Z s4lb L-lo-bc-20260801.xml (hashes equal to the list)
2026-11-10T05:40:23.140Z s4lb answered
2026-11-10T05:40:26.401Z s4r B-bills.xml B-payable.xml M-marks.xml N-0099.xml N-0101.xml R-0099.xml R-0101.xml (hashes equal to the list)
2026-11-10T05:40:28.028Z s4r answered
2026-11-10T05:40:49.962Z s5m M-marks.xml (hashes equal to the list)
2026-11-10T05:40:50.595Z s5m answered
2026-11-10T05:40:52.312Z s5 I-CN-1.xml (hashes equal to the list)
2026-11-10T05:40:57.926Z s5 answered
2026-11-10T05:41:12.734Z s5sa S-snap-20260401-20260801.xml (hashes equal to the list)
2026-11-10T05:41:13.565Z s5sa answered
2026-11-10T05:41:14.982Z s5sb S-snap-20260401-20260801.xml (hashes equal to the list)
2026-11-10T05:41:15.549Z s5sb answered
2026-11-10T05:41:16.711Z s5la L-lo-bc-20260801.xml (hashes equal to the list)
2026-11-10T05:41:22.506Z s5la answered
2026-11-10T05:41:23.842Z s5lb L-lo-bc-20260801.xml (hashes equal to the list)
2026-11-10T05:41:24.523Z s5lb answered
2026-11-10T05:41:27.471Z s5r B-bills.xml B-payable.xml M-marks.xml R-CN1.xml R-0099.xml R-0101.xml (hashes equal to the list)
2026-11-10T05:41:29.128Z s5r answered
2026-11-10T05:41:53.465Z s6m M-marks.xml (hashes equal to the list)
2026-11-10T05:41:54.405Z s6m answered
2026-11-10T05:41:57.269Z s6 I-CN-P.xml (hashes equal to the list)
2026-11-10T05:42:04.736Z s6 answered
2026-11-10T05:42:21.016Z s6sa S-snap-20260401-20260801.xml (hashes equal to the list)
2026-11-10T05:42:21.972Z s6sa answered
2026-11-10T05:42:23.454Z s6sb S-snap-20260401-20260801.xml (hashes equal to the list)
2026-11-10T05:42:24.313Z s6sb answered
2026-11-10T05:42:26.510Z s6la L-lo-bc-20260801.xml (hashes equal to the list)
2026-11-10T05:42:32.391Z s6la answered
2026-11-10T05:42:34.062Z s6lb L-lo-bc-20260801.xml (hashes equal to the list)
2026-11-10T05:42:34.773Z s6lb answered
2026-11-10T05:42:37.681Z s6r B-bills.xml B-payable.xml M-marks.xml R-CN2.xml R-CN1.xml R-0101.xml (hashes equal to the list)
2026-11-10T05:42:39.329Z s6r answered
2026-11-10T05:42:59.485Z s7m M-marks.xml (hashes equal to the list)
2026-11-10T05:43:01.082Z s7m answered
2026-11-10T05:43:04.244Z s7 I-CN-2.xml (hashes equal to the list)
2026-11-10T05:43:12.636Z s7 answered
2026-11-10T05:43:29.840Z s7sa S-snap-20260401-20260801.xml (hashes equal to the list)
2026-11-10T05:43:30.251Z s7sa answered
2026-11-10T05:43:31.079Z s7sb S-snap-20260401-20260801.xml (hashes equal to the list)
2026-11-10T05:43:31.865Z s7sb answered
2026-11-10T05:43:33.460Z s7la L-lo-bc-20260801.xml (hashes equal to the list)
2026-11-10T05:43:37.094Z s7la answered
2026-11-10T05:43:38.021Z s7lb L-lo-bc-20260801.xml (hashes equal to the list)
2026-11-10T05:43:38.515Z s7lb answered
2026-11-10T05:43:40.622Z s7r B-bills.xml B-payable.xml M-marks.xml R-CN3.xml R-CN1.xml R-0099.xml (hashes equal to the list)
2026-11-10T05:43:41.778Z s7r answered
2026-11-10T05:44:01.782Z s8m M-marks.xml (hashes equal to the list)
2026-11-10T05:44:02.176Z s8m answered
2026-11-10T05:44:04.572Z s8 I-CN-3.xml (hashes equal to the list)
2026-11-10T05:44:09.263Z s8 answered
2026-11-10T05:44:23.195Z s8sa S-snap-20260401-20260801.xml (hashes equal to the list)
2026-11-10T05:44:24.200Z s8sa answered
2026-11-10T05:44:25.436Z s8sb S-snap-20260401-20260801.xml (hashes equal to the list)
2026-11-10T05:44:26.020Z s8sb answered
2026-11-10T05:44:27.052Z s8la L-lo-bc-20260801.xml (hashes equal to the list)
2026-11-10T05:44:31.027Z s8la answered
2026-11-10T05:44:32.005Z s8lb L-lo-bc-20260801.xml (hashes equal to the list)
2026-11-10T05:44:32.554Z s8lb answered
2026-11-10T05:44:34.758Z s8r B-bills.xml B-payable.xml M-marks.xml R-CN4.xml R-CN1.xml R-0099.xml (hashes equal to the list)
2026-11-10T05:44:35.959Z s8r answered
2026-11-10T05:47:23.472Z s9 N-0099.xml N-0101.xml R-0099.xml R-0101.xml R-CN1.xml R-CN2.xml R-CN3.xml R-CN4.xml B-bills.xml B-payable.xml S-snap-20260401-20260801.xml M-marks.xml (hashes equal to the list)
2026-11-10T05:47:25.646Z s9 answered
```

## run.py logs (each send: local IST time to the second, request, answer file, curl exit, bytes)

### s1
```
11:06:48	M-marks.xml	a-s1-M-marks.xml	curl 0	3394 bytes
```

### s2
```
11:08:52	N-0099.xml	a-s2-N-0099.xml	curl 0	3022 bytes
11:08:53	N-0101.xml	a-s2-N-0101.xml	curl 0	3022 bytes
11:08:53	R-0099.xml	a-s2-R-0099.xml	curl 0	3022 bytes
11:08:53	R-0101.xml	a-s2-R-0101.xml	curl 0	3022 bytes
11:08:53	R-CN1.xml	a-s2-R-CN1.xml	curl 0	3022 bytes
11:08:53	R-CN2.xml	a-s2-R-CN2.xml	curl 0	3022 bytes
11:08:54	R-CN3.xml	a-s2-R-CN3.xml	curl 0	3022 bytes
11:08:54	R-CN4.xml	a-s2-R-CN4.xml	curl 0	3022 bytes
11:08:54	B-bills.xml	a-s2-B-bills.xml	curl 0	46 bytes
11:08:54	B-payable.xml	a-s2-B-payable.xml	curl 0	46 bytes
11:08:54	S-snap-20260401-20260801.xml	a-s2-S-snap-20260401-20260801.xml	curl 0	25480 bytes
11:08:54	M-marks.xml	a-s2-M-marks.xml	curl 0	3394 bytes
```

### s3m
```
11:09:16	M-marks.xml	a-s3m-M-marks.xml	curl 0	3394 bytes
```

### s3
```
11:09:30	L-ledger.xml	a-s3-L-ledger.xml	curl 0	538 bytes
```

### s4
```
11:09:36	I-0099.xml	a-s4-I-0099.xml	curl 0	540 bytes
```

### s4b
```
11:09:54	I-0101.xml	a-s4b-I-0101.xml	curl 0	540 bytes
```

### s4sa
```
11:10:11	S-snap-20260401-20260801.xml	a-s4sa-S-snap-20260401-20260801.xml	curl 0	26728 bytes
```

### s4sb
```
11:10:13	S-snap-20260401-20260801.xml	a-s4sb-S-snap-20260401-20260801.xml	curl 0	26728 bytes
```

### s4la
```
11:10:15	L-lo-bc-20260801.xml	a-s4la-L-lo-bc-20260801.xml	curl 0	1742 bytes
```

### s4lb
```
11:10:22	L-lo-bc-20260801.xml	a-s4lb-L-lo-bc-20260801.xml	curl 0	1742 bytes
```

### s4r
```
11:10:26	B-bills.xml	a-s4r-B-bills.xml	curl 0	970 bytes
11:10:26	B-payable.xml	a-s4r-B-payable.xml	curl 0	46 bytes
11:10:27	M-marks.xml	a-s4r-M-marks.xml	curl 0	3396 bytes
11:10:27	N-0099.xml	a-s4r-N-0099.xml	curl 0	5608 bytes
11:10:27	N-0101.xml	a-s4r-N-0101.xml	curl 0	5608 bytes
11:10:27	R-0099.xml	a-s4r-R-0099.xml	curl 0	95834 bytes
11:10:27	R-0101.xml	a-s4r-R-0101.xml	curl 0	95834 bytes
```

### s5m
```
11:10:50	M-marks.xml	a-s5m-M-marks.xml	curl 0	3396 bytes
```

### s5
```
11:10:52	I-CN-1.xml	a-s5-I-CN-1.xml	curl 0	540 bytes
```

### s5sa
```
11:11:13	S-snap-20260401-20260801.xml	a-s5sa-S-snap-20260401-20260801.xml	curl 0	26728 bytes
```

### s5sb
```
11:11:15	S-snap-20260401-20260801.xml	a-s5sb-S-snap-20260401-20260801.xml	curl 0	26728 bytes
```

### s5la
```
11:11:17	L-lo-bc-20260801.xml	a-s5la-L-lo-bc-20260801.xml	curl 0	896 bytes
```

### s5lb
```
11:11:24	L-lo-bc-20260801.xml	a-s5lb-L-lo-bc-20260801.xml	curl 0	896 bytes
```

### s5r
```
11:11:27	B-bills.xml	a-s5r-B-bills.xml	curl 0	510 bytes
11:11:28	B-payable.xml	a-s5r-B-payable.xml	curl 0	46 bytes
11:11:28	M-marks.xml	a-s5r-M-marks.xml	curl 0	3396 bytes
11:11:28	R-CN1.xml	a-s5r-R-CN1.xml	curl 0	95834 bytes
11:11:28	R-0099.xml	a-s5r-R-0099.xml	curl 0	95834 bytes
11:11:28	R-0101.xml	a-s5r-R-0101.xml	curl 0	95834 bytes
```

### s6m
```
11:11:54	M-marks.xml	a-s6m-M-marks.xml	curl 0	3396 bytes
```

### s6
```
11:11:57	I-CN-P.xml	a-s6-I-CN-P.xml	curl 0	540 bytes
```

### s6sa
```
11:12:21	S-snap-20260401-20260801.xml	a-s6sa-S-snap-20260401-20260801.xml	curl 0	26724 bytes
```

### s6sb
```
11:12:23	S-snap-20260401-20260801.xml	a-s6sb-S-snap-20260401-20260801.xml	curl 0	26724 bytes
```

### s6la
```
11:12:27	L-lo-bc-20260801.xml	a-s6la-L-lo-bc-20260801.xml	curl 0	1282 bytes
```

### s6lb
```
11:12:34	L-lo-bc-20260801.xml	a-s6lb-L-lo-bc-20260801.xml	curl 0	1282 bytes
```

### s6r
```
11:12:38	B-bills.xml	a-s6r-B-bills.xml	curl 0	508 bytes
11:12:38	B-payable.xml	a-s6r-B-payable.xml	curl 0	46 bytes
11:12:38	M-marks.xml	a-s6r-M-marks.xml	curl 0	3396 bytes
11:12:38	R-CN2.xml	a-s6r-R-CN2.xml	curl 0	95830 bytes
11:12:38	R-CN1.xml	a-s6r-R-CN1.xml	curl 0	95834 bytes
11:12:38	R-0101.xml	a-s6r-R-0101.xml	curl 0	95834 bytes
```

### s7m
```
11:13:00	M-marks.xml	a-s7m-M-marks.xml	curl 0	3396 bytes
```

### s7
```
11:13:04	I-CN-2.xml	a-s7-I-CN-2.xml	curl 0	540 bytes
```

### s7sa
```
11:13:30	S-snap-20260401-20260801.xml	a-s7sa-S-snap-20260401-20260801.xml	curl 0	26724 bytes
```

### s7sb
```
11:13:31	S-snap-20260401-20260801.xml	a-s7sb-S-snap-20260401-20260801.xml	curl 0	26724 bytes
```

### s7la
```
11:13:33	L-lo-bc-20260801.xml	a-s7la-L-lo-bc-20260801.xml	curl 0	2906 bytes
```

### s7lb
```
11:13:38	L-lo-bc-20260801.xml	a-s7lb-L-lo-bc-20260801.xml	curl 0	2906 bytes
```

### s7r
```
11:13:40	B-bills.xml	a-s7r-B-bills.xml	curl 0	508 bytes
11:13:41	B-payable.xml	a-s7r-B-payable.xml	curl 0	508 bytes
11:13:41	M-marks.xml	a-s7r-M-marks.xml	curl 0	3398 bytes
11:13:41	R-CN3.xml	a-s7r-R-CN3.xml	curl 0	95836 bytes
11:13:41	R-CN1.xml	a-s7r-R-CN1.xml	curl 0	95836 bytes
11:13:41	R-0099.xml	a-s7r-R-0099.xml	curl 0	95836 bytes
```

### s8m
```
11:14:02	M-marks.xml	a-s8m-M-marks.xml	curl 0	3398 bytes
```

### s8
```
11:14:04	I-CN-3.xml	a-s8-I-CN-3.xml	curl 0	540 bytes
```

### s8sa
```
11:14:24	S-snap-20260401-20260801.xml	a-s8sa-S-snap-20260401-20260801.xml	curl 0	26724 bytes
```

### s8sb
```
11:14:25	S-snap-20260401-20260801.xml	a-s8sb-S-snap-20260401-20260801.xml	curl 0	26724 bytes
```

### s8la
```
11:14:27	L-lo-bc-20260801.xml	a-s8la-L-lo-bc-20260801.xml	curl 0	3720 bytes
```

### s8lb
```
11:14:32	L-lo-bc-20260801.xml	a-s8lb-L-lo-bc-20260801.xml	curl 0	3720 bytes
```

### s8r
```
11:14:35	B-bills.xml	a-s8r-B-bills.xml	curl 0	508 bytes
11:14:35	B-payable.xml	a-s8r-B-payable.xml	curl 0	966 bytes
11:14:35	M-marks.xml	a-s8r-M-marks.xml	curl 0	3398 bytes
11:14:35	R-CN4.xml	a-s8r-R-CN4.xml	curl 0	95836 bytes
11:14:35	R-CN1.xml	a-s8r-R-CN1.xml	curl 0	95836 bytes
11:14:35	R-0099.xml	a-s8r-R-0099.xml	curl 0	95836 bytes
```

### s9
```
11:17:23	N-0099.xml	a-s9-N-0099.xml	curl 0	3022 bytes
11:17:24	N-0101.xml	a-s9-N-0101.xml	curl 0	3022 bytes
11:17:24	R-0099.xml	a-s9-R-0099.xml	curl 0	3022 bytes
11:17:24	R-0101.xml	a-s9-R-0101.xml	curl 0	3022 bytes
11:17:24	R-CN1.xml	a-s9-R-CN1.xml	curl 0	3022 bytes
11:17:24	R-CN2.xml	a-s9-R-CN2.xml	curl 0	3022 bytes
11:17:24	R-CN3.xml	a-s9-R-CN3.xml	curl 0	3022 bytes
11:17:25	R-CN4.xml	a-s9-R-CN4.xml	curl 0	3022 bytes
11:17:25	B-bills.xml	a-s9-B-bills.xml	curl 0	46 bytes
11:17:25	B-payable.xml	a-s9-B-payable.xml	curl 0	46 bytes
11:17:25	S-snap-20260401-20260801.xml	a-s9-S-snap-20260401-20260801.xml	curl 0	25480 bytes
11:17:25	M-marks.xml	a-s9-M-marks.xml	curl 0	3394 bytes
```
