# Row 2346-TD: master's own bills read, at the period's last day, showed the same three bills with the same BILLCL and the same closing as at the notes' date; only BILLOVERDUE moved (0 to 242)

Run on 10 Nov 2026, 07:00:18Z to 07:12:46Z (12:30 to 12:42 IST), on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, as rows 2346-AR, 2377-NB and 2346-PB read it from the program's file; the Gateway shows no release number), by the sheet at `lab/2346-td-sheet` (bafec1aa), on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic), loaded alone in a sitting of its own. Seven imports, each file sent once, on the run lead's yes to the row's C3 exception given in the queue line; read-only reads between them, one at a time through `run.py` (curl, each answer's headers kept); no Bridge build. Then the restore of this sitting's own backup. Every value below is M, measured by this run, in one run. Every read was answered, each within about 2 s (`evidence/gaps.txt`); no answer was empty, so the empty-answer exception was never needed. The window after each import and after each named shot was the Gateway, no dialog.

This sitting ran after row 683-RO (Tally quit at 06:49:35Z), with a Tally start of its own.

## Result

PARTIAL by the sheet's Decides: one book, one release, one sitting, master's requests sent by curl, without the reads Bridge sends around them.

**The rebuild (steps 3 and 4): PB's state, with the one change of name.**

| import | step | `CREATED` | `ERRORS` | `EXCEPTIONS` | `LASTVCHID` | against PB's answer (`td-diff.py --map`) |
|---|---|---|---|---|---|---|
| `L-ledger.xml`, `BRIDGE Walkin BC` | s3 | 1 | 0 | 0 | 0 | EQUAL |
| `I-0099.xml`, `BP/26-27/0099` | s4 | 1 | 0 | 0 | 45 | EQUAL |
| `I-0101.xml`, `BP/26-27/0101` | s4b | 1 | 0 | 0 | 46 | EQUAL |
| `I-CN-1.xml`, c1 | s5 | 1 | 0 | 0 | 47 | EQUAL |
| `I-CN-P.xml`, p | s6 | 1 | 0 | 0 | 48 | EQUAL |
| `I-CN-2.xml`, c2 | s7 | 1 | 0 | 0 | 49 | EQUAL |
| `I-CN-3.xml`, c3 | s8 | 1 | 0 | 0 | 50 | EQUAL |

Each import answer was byte-equal to PB's (no name in them to map). Each marks read before a write passed the before-write check against s1 (ALTVCHID 68, 68, 69, 70, 71, 72, 73; ALTMSTID 245, then 246 from the ledger on).

**s8r, the state:** it is PB's state, by each condition of step 4.
- R-CN1 to R-CN4 one row each, Credit Notes `1` to `4` in the order sent (CN4's bill `BP/26-27/0098`, `New Ref`, 1051.00).
- R-0099 and R-0101 one row each.
- B-bills: one group for `BRIDGE Walkin BC`, `BP/26-27/0101` -525.50.
- B-payable: two, `BP/26-27/0099` and `BP/26-27/0098`, 1051.00 each.
- Marks: ALTVCHID 74, ALTMSTID 246.
- `td-diff.py --map` (`evidence/diff-s8r.txt`):
  - R-CN4, R-CN1, R-0099, B-bills, B-payable and the marks are EQUAL to PB's s8r answers.
  - R-CN2 (against PB's s6r), R-CN3 (against PB's s7r) and R-0101 (against PB's s6r) DIFFER, 10 lines each. Every one of those lines is a CMPINFO count line (LEDGER, CURRENCY, TAXUNIT, VOUCHERNUMBERSERIES and VOUCHER). PB's answers there were read before its later notes; here all four notes were in the book.

**The bills read (steps 5 and 6).** Each read was taken in `fetch_party_bills`'s order and gave the following:
- the two snapshot answers byte-equal, and the two report answers byte-equal;
- each answer HTTP 200 with one `Content-Type` (`text/xml; charset=utf-16`);
- `pb.py --outcome` told.

| read | to-date | the report's rows (`BILLREF`, `BILLCL`; each dated and due 1-Aug-26) | `BILLOVERDUE` | sum of `BILLCL` | snapshot closing | report, snapshot bytes | `pb.py --outcome` |
|---|---|---|---|---|---|---|---|
| control, c1 | 20260801 | `BP/26-27/0099` 1051.00; `BP/26-27/0101` -525.50; `BP/26-27/0098` 1051.00 | 0 on each | 1576.50 | 1576.50 | 3720 B, 26720 B | told, ROWS 3 (0 On Account) |
| case, t1 | 20270331 | the same three, the same `BILLCL` | 242 on each | 1576.50 | 1576.50 | 3732 B, 26762 B | told, ROWS 3 (0 On Account) |

- **The control against PB (`evidence/diff-c1.txt`):** the snapshot is EQUAL to PB's s8sa with the name mapped (2 places). The report is byte-equal to PB's s8la: the report does not carry the name, so nothing was mapped.
- **The case against the control (`evidence/diff-t1.txt`):**
  - **The report differs only in `BILLOVERDUE`,** 0 to 242 on each of the three bills. 242 is the number of days from 1-Aug-26 to 31-Mar-27.
  - **The snapshot keeps the customer's row** (`BRIDGE Walkin BC`, closing 1576.50, the same 19 rows).
  - **It differs in 13 other ledgers' `CLOSINGBALANCE`** (`evidence/snap-ledgers-t1-c1.txt`): the book's vouchers dated after 1 Aug 2026 (the date of last entry is 15-Sep-26), in ledgers other than the customer's.
- **The case's own time:** the snapshot to 20270331 answered in about 0.6 s, as the control's (`evidence/gaps.txt`). The sheet records its time as part of the result.

**m1 (step 7):** equal to s8r (74/246): steps 5 and 6 only read.

By the sheet's Decides, quoted:
- **Settled before the row:** "`NativeLedgerSnapshotPeriod` admits 20260401 to 20270331 under both profiles."
- **The control, told, and both diffs EQUAL:** "master's own request, through `LedgerScopeName`, reads PB's state. PB's findings on a settled, a part-settled and an over-settled bill then stand for a request master sends."
- **The case, told, the same rows with the same `BILLCL`, and the same closing:** "on this book, the read at the period's last day shows the same bills a note is checked against when nothing is dated after it. So the builder can take its to-date as check 4 says, and checks 1 to 3 are judged on the same rows. A later voucher is still not measured."
- Recorded, not judged: `BILLOVERDUE` moved with the to-date (the sheet does not estimate it at a to-date after Tally's current date), and the snapshot's other ledgers moved with the vouchers dated after 1 Aug 2026.

**The way back (step 8).** BK was restored by C6 with only the two expected prompts. Then s9 sent every file of s2, in s2's order:
- `--same-row` against s1: equal (ALTVCHID 68, ALTMSTID 245);
- each N and R read: 0 rows;
- B-bills and B-payable: 0 groups for `BRIDGE Walkin BC`;
- each snapshot: no row for it;
- each of the 13 answers: sha256 equal to s2's (`evidence/s9-vs-s2.txt`).

**The sheet's estimates:** s2, steps 3 and 4, the control, the case's rows, `BILLCL` and closing, and s9 each as estimated. The three read-backs compared with PB's earlier reads differed only in CMPINFO counts, as above.

**Deviations from the sheet and the lab's rules:**
1. **Bridge processes.** Three Bridge processes were running at the start. On the run lead's standing word none was ended, and they were left running at the close. No Bridge tool was called and no request went through Bridge.
2. **The start-up company.** Tally's start-up company loaded by itself, as the install is set to. It was shut by the K: Company label, Shut, Enter on it, before step 0, with no change to it.

**Beyond the sheet, recorded and not judged:**
- B-bills and B-payable answered 46 B, an empty `ENVELOPE`, at s2 and s9.
- The snapshots answered 25480 B (to 20260801) and 25518 B (to 20270331) at s2 and s9, with 18 `LEDGER` rows and no row for the customer. With the customer's row, they answered 26720 B and 26762 B (19 rows).
- `gen-td.py` was run again before the start, in a new folder holding copies of itself and `pb/`. Its seven files were byte-equal to the sheet's.
- The Gateway's company line carried "(e)" and "(Schedule Backup)" at step 1.

## Step 1: load, check and back up
- Sheet files: all 58 equal to `SHA256.txt` at bafec1aa (sizes too). These are byte-equal to row 2346-PB's (`lab/2346-pb-sheet` 53540665):
  - `.gitattributes`, `LAB-COMMON.md`, `run.py`, `M-marks.xml`;
  - the N, R and B reads and `S-snap-20260401-20260801.xml`;
  - the eight files in `pb/`.
- The 19 files in `pb-answers/` are byte-equal to PB's answers of the same names.
- `pb.py` and `count20.py` each differ from PB's by the TD-marked line (and `count20.py`'s header line).
- The two request pairs differ only in `SVTODATE`, and `L-lo-wk-20260801.xml` differs from PB's report request only by the hyphen.
- Before the start, the book's data folder was found as the one whose `Company.1800` holds BRIDGE PILOT LAB and the book's GUID (one folder); it does not hold the excluded GUID (`evidence/bookcheck.txt`). The backup folder was made new and empty.
- Tally was started once, at 07:00:18Z, from its desktop shortcut, after tasklist showed none and nothing listened on 9001. Its start-up company was shut first (deviation 2).
- C2 at 07:01:21Z, with Select Company showing: one `tally.exe`, which alone owns 9001; `GET /status` 200 (51 B); no company loaded.
- The book was loaded alone: `BRIDGE`, space, `PILOT`, space, `LAB` typed into Select Company's name filter, one row, BRIDGE PILOT LAB, 1-Apr-26 to 31-Mar-27 (`s1-select`), Enter, no prompt. The Gateway (`s1-gateway`): current period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026 (not before 1-Aug-2026); date of last entry 15-Sep-26. The PC's date: 10 Nov 2026 (IST).
- `s1`: one COMPANY row, the book's GUID, ALTVCHID 68, ALTMSTID 245; EQUAL to PB's s1. EXPECT met.
- BK by C5 (07:02Z to 07:03Z) to `C:\lab\2346-td\backup-1` (Tally showed `C:\lab\2346-Td\backup-1`, as the sheet says it may). The keys:
  - Data label, Backup & Restore, Backup; Backspace to the path;
  - Specify Path (Up 3 times); the path typed in two chunks; Enter, then Enter on the folder row;
  - `BRIDGE`, space, `PILOT`, space, `LAB` typed, the one row, Enter; End of List (Up 3 times), Enter;
  - "Accept ?" (`bk-accept`, 07:03:38Z) = y. No other message.

  One file, `TDBK1800_<company number>.001`, 90,051 B, sha256 `dfe530cd2c94c7a07ac0aedfdb76a55e140eab3d3d490338435e7a7d17a59725`. The Restore list showed it as 10-Nov-26 (12:33 pm).

## Step 2: the book before
- `s2` (07:04:05Z): each N and R read 0 rows; B-bills and B-payable 0 groups for `BRIDGE Walkin BC`; each snapshot no row for it; M as s1. EXPECT met.

## Steps 3 and 4: the rebuild and the state
- Each write by the sheet's "Each write" (the row's `wr.sh`):
  - `<step>m` and the before-write check against s1 (`evidence/bw-<step>.txt`);
  - the import alone, and the window captured at once (`s3-after` to `s8-after`): the Gateway, no dialog, each time;
  - `count20.py` on the answer (`evidence/count-<step>.txt`), and `td-diff.py --map` against PB's (`evidence/diff-<step>.txt`).
- `s8r` (07:06:49Z) as in the Result (`evidence/count-s8r.txt`, `evidence/diff-s8r.txt`).

## Steps 5 to 7: the reads
- Each read by the row's `br-td.sh`: `<p>sa`, `<p>sb`, `<p>la` (the window captured at once, `<p>la-after`), `<p>lb`, then `pb.py --outcome` (`evidence/outcome-c1.txt`, `evidence/outcome-t1.txt`). For the case, the window was also captured at once after `t1sa` (`t1sa-after`). The window was the Gateway, with no dialog, at each capture.
- The control (07:07:26Z to 07:07:38Z) and the case (07:07:57Z to 07:08:14Z): as in the Result.
- `m1` (07:08:49Z): `count20.py --same-row` against s8r, equal (`evidence/same-m1.txt`).

## Step 8: restore (C6) and compare
- Before the shut, `prep-c6.sh 8`:
  - the BK re-hashed equal to `evidence/bk.txt`, its folder holding only that file;
  - `restore-plan.txt` written;
  - the data folder listed (24 entries).

  The plan and listings stay on the lab PC.
- The restore, key by key:
  - K: Company label, Shut, Enter on BRIDGE PILOT LAB, no prompt (`c6-1-after-shut`);
  - Data label, Backup & Restore, Restore. Restore Data Path offered `<Tally data folder>`; Specify Path (Up 3 times); the path typed in two chunks;
  - the list (`8-restore-list`) showed one row, BRIDGE PILOT LAB (`<company number>`), 10-Nov-26 (12:33 pm), as the plan. Enter on it; End of List, Enter;
  - "Accept ?" (`8-restore-accept`, 07:10:44Z) = y;
  - "Company data folder (`<company number>`) already exists." / "Selecting Skip will cancel the Restore process for this company." / "Selecting Replace will overwrite the existing folder.", buttons "R Replace" and "S Skip" (`8-restore-replace`, 07:10:59Z) = r. Select Company followed, no other message.
- The data folder after the restore: after=17, before=24 entries. Gone: `CmpSchMetaData.1800`, `Index.1800`, `TCMPSCHACCESS.TSF`, `TCMPSCHMESSAGE.TSF`, `TCMPSCHSTATE.TSF`, `TCMPSCHUPDATE.TSF`, `TINTMSG.TSF`. None new.
- Loaded alone: `BRIDGE`, space, `PILOT`, space, `LAB` typed, one row (`8-select`), Enter, no prompt. The Gateway (`8-gateway`): period 1-Apr-26 to 31-Mar-27, current date Tuesday, 15-Sep-2026, date of last entry 15-Sep-26: as at step 1.
- `s9` (07:11:51Z): as in the Result (`evidence/same-s9.txt`, `evidence/s9-vs-s2.txt`, `evidence/count-s9.txt`, `evidence/pb-s9.txt`).

## Step 9: close (C10)
- Shut by the K: Company label, Shut, BRIDGE PILOT LAB alone, Enter, no prompt: Select Company, no company loaded (`c10-no-company`, 07:12:28Z).
- C2 at 07:12:28Z, after the last request: one `tally.exe` (the same process as at step 0), which alone owns 9001; `GET /status` 200 (51 B). No XML request after the shut.
- Tally was then closed by its own Quit (Q: Quit clicked at Select Company), no prompt; at 07:12:46Z no `tally.exe` ran and nothing listened on 9001. The three Bridge processes were left running (deviation 1).

## Files
- `answers/`: every answer and its headers file that may go, byte for byte (`kept-local.txt` lists those that stay on the lab PC, by size and sha256, with the reason). The sheet's `pb-answers/` copies are not repeated here.
- `evidence/`: notes, every `count20.py` and `pb.py` output, the before-write checks, the outcomes, compares and diffs, the consoles of s8r, c1, t1 and s9, the snapshot ledger list, the gaps between sends, and the answers' sizes and sha256. Process and port listings, the backup-folder listings and the data-folder listings stay on the lab PC. The BK record (`bk.txt`) goes with the company number replaced.
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
  groups for 'BRIDGE Walkin BC': 0
== a-s2-B-payable.xml: 46 bytes
  STATUS <absent>
  BILLFIXED groups: 0
  groups for 'BRIDGE Walkin BC': 0
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

### s4m
```
== a-s4m-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
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

### s4bm
```
== a-s4bm-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
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
== a-s8r-R-CN4.xml: 95832 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '4'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walkin BC'
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
   leg 'BRIDGE Walkin BC' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walkin BC': 1 with NAME, 0 whitespace-only, 1 lists
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
== a-s8r-R-CN1.xml: 95832 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walkin BC'
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
   leg 'BRIDGE Walkin BC' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walkin BC': 1 with NAME, 0 whitespace-only, 1 lists
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
== a-s8r-R-CN2.xml: 95828 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '2'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0101'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walkin BC'
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
   leg 'BRIDGE Walkin BC' | 'No' | '525.50'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-300.20'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-200.20'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-12.51'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-12.51'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.08'
   bills on 'BRIDGE Walkin BC': 1 with NAME, 0 whitespace-only, 1 lists
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
== a-s8r-R-CN3.xml: 95832 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '3'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walkin BC'
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
   leg 'BRIDGE Walkin BC' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walkin BC': 1 with NAME, 0 whitespace-only, 1 lists
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
== a-s8r-R-0099.xml: 95832 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0099'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walkin BC'
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
   leg 'BRIDGE Walkin BC' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   bills on 'BRIDGE Walkin BC': 1 with NAME, 0 whitespace-only, 1 lists
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
== a-s8r-R-0101.xml: 95832 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0101'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0101'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walkin BC'
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
   leg 'BRIDGE Walkin BC' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   bills on 'BRIDGE Walkin BC': 1 with NAME, 0 whitespace-only, 1 lists
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
== a-s8r-B-bills.xml: 506 bytes
  STATUS <absent>
  BILLFIXED groups: 1
  groups for 'BRIDGE Walkin BC': 1
   BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50'
== a-s8r-B-payable.xml: 962 bytes
  STATUS <absent>
  BILLFIXED groups: 2
  groups for 'BRIDGE Walkin BC': 2
   BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00'
   BILLREF 'BP/26-27/0098' BILLDATE '1-Aug-26' BILLCL '1051.00'
== a-s8r-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '74' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### m1
```
== a-m1-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '74' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
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
  groups for 'BRIDGE Walkin BC': 0
== a-s9-B-payable.xml: 46 bytes
  STATUS <absent>
  BILLFIXED groups: 0
  groups for 'BRIDGE Walkin BC': 0
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
  'BRIDGE Walkin BC': no row
a-s2-S-snap-20260401-20270331.xml: HEADER/STATUS 1
  LEDGER rows: 18
  'BRIDGE Walkin BC': no row
exit 0
```

### c1sa
```
a-c1sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walkin BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '1576.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 1576.50
exit 0
```

### c1sb
```
a-c1sb-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walkin BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '1576.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 1576.50
exit 0
```

### c1la
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['3720']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0098' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: 1576.50
ANSWER: ROWS 3 (0 On Account)
exit 0
```

### c1lb
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['3720']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0098' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: 1576.50
ANSWER: ROWS 3 (0 On Account)
exit 0
```

### t1sa
```
a-t1sa-S-snap-20260401-20270331.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walkin BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '1576.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 1576.50
exit 0
```

### t1sb
```
a-t1sb-S-snap-20260401-20270331.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walkin BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '1576.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 1576.50
exit 0
```

### t1la
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['3732']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '242'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '242'
  BILLREF 'BP/26-27/0098' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '242'
  sum of BILLCL: 1576.50
ANSWER: ROWS 3 (0 On Account)
exit 0
```

### t1lb
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['3732']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '242'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '242'
  BILLREF 'BP/26-27/0098' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '242'
  sum of BILLCL: 1576.50
ANSWER: ROWS 3 (0 On Account)
exit 0
```

### s9
```
a-s9-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 18
  'BRIDGE Walkin BC': no row
a-s9-S-snap-20260401-20270331.xml: HEADER/STATUS 1
  LEDGER rows: 18
  'BRIDGE Walkin BC': no row
exit 0
```

## pb.py --outcome, whole

### the control, at 20260801
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26720']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26720']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['3720']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['3720']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0098' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: 1576.50
ANSWER: ROWS 3 (0 On Account)
a-c1sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walkin BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '1576.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 1576.50
OUTCOME: told (ROWS 3 (0 On Account), sum of BILLCL 1576.50 equals closing 1576.50)
exit 0
```

### the case, at 20270331
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26762']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26762']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['3732']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['3732']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '242'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '242'
  BILLREF 'BP/26-27/0098' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '242'
  sum of BILLCL: 1576.50
ANSWER: ROWS 3 (0 On Account)
a-t1sa-S-snap-20260401-20270331.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walkin BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '1576.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 1576.50
OUTCOME: told (ROWS 3 (0 On Account), sum of BILLCL 1576.50 equals closing 1576.50)
exit 0
```

## Compares, checks and consoles, whole

### s1: td-diff.py --map against PB's s1
```
a-s1-M-marks.xml: 3394 B sha256 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
pb-answers/a-s1-M-marks.xml: 3394 B sha256 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
bytes as they are: equal
name mapped: 0 place(s) in pb-answers/a-s1-M-marks.xml
EQUAL
exit 0
```

### s3m: M against s1 (the before-write check)
```
== a-s3m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s3: td-diff.py --map against PB's answer
```
a-s3-L-ledger.xml: 538 B sha256 1b72152a8fd20ccb9366277864bf32357b8891e536cde2a54992adaa60a97010
pb-answers/a-s3-L-ledger.xml: 538 B sha256 1b72152a8fd20ccb9366277864bf32357b8891e536cde2a54992adaa60a97010
bytes as they are: equal
name mapped: 0 place(s) in pb-answers/a-s3-L-ledger.xml
EQUAL
exit 0
```

### s4m: M against s1 (the before-write check)
```
== a-s4m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4: td-diff.py --map against PB's answer
```
a-s4-I-0099.xml: 540 B sha256 fdcd8435c6fcf95ab624cbe2cff2c887078df500626f7c9c37cfa834a1cdb6ea
pb-answers/a-s4-I-0099.xml: 540 B sha256 fdcd8435c6fcf95ab624cbe2cff2c887078df500626f7c9c37cfa834a1cdb6ea
bytes as they are: equal
name mapped: 0 place(s) in pb-answers/a-s4-I-0099.xml
EQUAL
exit 0
```

### s4bm: M against s1 (the before-write check)
```
== a-s4bm-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s4b: td-diff.py --map against PB's answer
```
a-s4b-I-0101.xml: 540 B sha256 32e7a29b201c773547aa47bc95bd3eda8af83aeb6e35090303b9b27bcd34eec5
pb-answers/a-s4b-I-0101.xml: 540 B sha256 32e7a29b201c773547aa47bc95bd3eda8af83aeb6e35090303b9b27bcd34eec5
bytes as they are: equal
name mapped: 0 place(s) in pb-answers/a-s4b-I-0101.xml
EQUAL
exit 0
```

### s5m: M against s1 (the before-write check)
```
== a-s5m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s5: td-diff.py --map against PB's answer
```
a-s5-I-CN-1.xml: 540 B sha256 f8384a874e1d8a6be92c2969f94e2887fb700768f6e111b6b3f649cd13ffdc7e
pb-answers/a-s5-I-CN-1.xml: 540 B sha256 f8384a874e1d8a6be92c2969f94e2887fb700768f6e111b6b3f649cd13ffdc7e
bytes as they are: equal
name mapped: 0 place(s) in pb-answers/a-s5-I-CN-1.xml
EQUAL
exit 0
```

### s6m: M against s1 (the before-write check)
```
== a-s6m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s6: td-diff.py --map against PB's answer
```
a-s6-I-CN-P.xml: 540 B sha256 e6540f6534095f5608f3c59f802c1f40a3ae13529a7e9041847b460f03c32fec
pb-answers/a-s6-I-CN-P.xml: 540 B sha256 e6540f6534095f5608f3c59f802c1f40a3ae13529a7e9041847b460f03c32fec
bytes as they are: equal
name mapped: 0 place(s) in pb-answers/a-s6-I-CN-P.xml
EQUAL
exit 0
```

### s7m: M against s1 (the before-write check)
```
== a-s7m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s7: td-diff.py --map against PB's answer
```
a-s7-I-CN-2.xml: 540 B sha256 93e331323bb1235565ab31f2ddbbaf7afaa752ce6770d0ae7ae145d7b3244234
pb-answers/a-s7-I-CN-2.xml: 540 B sha256 93e331323bb1235565ab31f2ddbbaf7afaa752ce6770d0ae7ae145d7b3244234
bytes as they are: equal
name mapped: 0 place(s) in pb-answers/a-s7-I-CN-2.xml
EQUAL
exit 0
```

### s8m: M against s1 (the before-write check)
```
== a-s8m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'73'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
exit 0
```

### s8: td-diff.py --map against PB's answer
```
a-s8-I-CN-3.xml: 540 B sha256 200c17e120c3ff582d28c9fce94b127ed22aaa4e5d10bbdd1d18e97a767f40c3
pb-answers/a-s8-I-CN-3.xml: 540 B sha256 200c17e120c3ff582d28c9fce94b127ed22aaa4e5d10bbdd1d18e97a767f40c3
bytes as they are: equal
name mapped: 0 place(s) in pb-answers/a-s8-I-CN-3.xml
EQUAL
exit 0
```

### s8r: send.sh console
```
R-CN4.xml: 95832 bytes
R-CN1.xml: 95832 bytes
R-CN2.xml: 95828 bytes
R-CN3.xml: 95832 bytes
R-0099.xml: 95832 bytes
R-0101.xml: 95832 bytes
B-bills.xml: 506 bytes
B-payable.xml: 962 bytes
M-marks.xml: 3398 bytes
== a-s8r-R-CN4.xml: 95832 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '4'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walkin BC'
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
   leg 'BRIDGE Walkin BC' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walkin BC': 1 with NAME, 0 whitespace-only, 1 lists
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
== a-s8r-R-CN1.xml: 95832 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '1'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walkin BC'
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
   leg 'BRIDGE Walkin BC' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walkin BC': 1 with NAME, 0 whitespace-only, 1 lists
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
== a-s8r-R-CN2.xml: 95828 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '2'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0101'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walkin BC'
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
   leg 'BRIDGE Walkin BC' | 'No' | '525.50'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-300.20'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-200.20'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-12.51'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-12.51'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.08'
   bills on 'BRIDGE Walkin BC': 1 with NAME, 0 whitespace-only, 1 lists
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
== a-s8r-R-CN3.xml: 95832 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER '3'
   VOUCHERTYPENAME 'Credit Note'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walkin BC'
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
   leg 'BRIDGE Walkin BC' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walkin BC': 1 with NAME, 0 whitespace-only, 1 lists
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
== a-s8r-R-0099.xml: 95832 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0099'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0099'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walkin BC'
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
   leg 'BRIDGE Walkin BC' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   bills on 'BRIDGE Walkin BC': 1 with NAME, 0 whitespace-only, 1 lists
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
== a-s8r-R-0101.xml: 95832 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'BP/26-27/0101'
   VOUCHERTYPENAME 'BRIDGE Sales'
   REFERENCE 'BP/26-27/0101'
   REFERENCEDATE '20260801'
   PARTYLEDGERNAME 'BRIDGE Walkin BC'
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
   leg 'BRIDGE Walkin BC' | 'Yes' | '-1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'No' | '600.40'
   leg 'BRIDGE Svc 998314 5%' | 'No' | '400.40'
   leg 'BRIDGE CGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE SGST 2.5%' | 'No' | '25.02'
   leg 'BRIDGE Round Off' | 'No' | '0.16'
   bills on 'BRIDGE Walkin BC': 1 with NAME, 0 whitespace-only, 1 lists
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
== a-s8r-B-bills.xml: 506 bytes
  STATUS <absent>
  BILLFIXED groups: 1
  groups for 'BRIDGE Walkin BC': 1
   BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50'
== a-s8r-B-payable.xml: 962 bytes
  STATUS <absent>
  BILLFIXED groups: 2
  groups for 'BRIDGE Walkin BC': 2
   BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00'
   BILLREF 'BP/26-27/0098' BILLDATE '1-Aug-26' BILLCL '1051.00'
== a-s8r-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '74' ALTMSTID '246'
  GUID is the book's: yes
  COMPANY rows: 1
exit 0
```

### s8r: td-diff.py --map against PB's answers
```
$ td-diff.py --map a-s8r-R-CN4.xml pb-answers/a-s8r-R-CN4.xml
a-s8r-R-CN4.xml: 95832 B sha256 90b932dc8003ab78509567410dbb41031d30de28272704028cfca705ce36a264
pb-answers/a-s8r-R-CN4.xml: 95836 B sha256 8704315d4ea8dadc4e412cd1cbc320a0b43d82e792fdab6944dae3731dbb42bb
bytes as they are: not equal
name mapped: 2 place(s) in pb-answers/a-s8r-R-CN4.xml
EQUAL
exit 0
$ td-diff.py --map a-s8r-R-CN1.xml pb-answers/a-s8r-R-CN1.xml
a-s8r-R-CN1.xml: 95832 B sha256 1ddb030cd305b29106759c8dd41ff9433445eb6996de99537598387e720aa3b1
pb-answers/a-s8r-R-CN1.xml: 95836 B sha256 6e5de6d41f4d687fed631808c6a8369f4300504e9d8fcd7fef13d2994bd6e26e
bytes as they are: not equal
name mapped: 2 place(s) in pb-answers/a-s8r-R-CN1.xml
EQUAL
exit 0
$ td-diff.py --map a-s8r-R-0099.xml pb-answers/a-s8r-R-0099.xml
a-s8r-R-0099.xml: 95832 B sha256 b5af11b23955c6cf9142d97afa3283b89fc1e9a963a9374b71b0fafa98a13ca4
pb-answers/a-s8r-R-0099.xml: 95836 B sha256 70844f32ad54d3eaac4042d701c12ae6c730f70bf3289c3c146e82468557bf5c
bytes as they are: not equal
name mapped: 2 place(s) in pb-answers/a-s8r-R-0099.xml
EQUAL
exit 0
$ td-diff.py --map a-s8r-B-bills.xml pb-answers/a-s8r-B-bills.xml
a-s8r-B-bills.xml: 506 B sha256 a43ee72c3ffb4a03fea640e6ba4284f8c8197e13de2fd1513f834305b95fd5af
pb-answers/a-s8r-B-bills.xml: 508 B sha256 5503268c3303f4b75cdc949d3a68768e141de143eadaf98a42b4469c4e7fa823
bytes as they are: not equal
name mapped: 1 place(s) in pb-answers/a-s8r-B-bills.xml
EQUAL
exit 0
$ td-diff.py --map a-s8r-B-payable.xml pb-answers/a-s8r-B-payable.xml
a-s8r-B-payable.xml: 962 B sha256 96b5e3c7c5fc8c9d75443684cad5ad44dd419041795c59c622085780a70de90e
pb-answers/a-s8r-B-payable.xml: 966 B sha256 1e6171e57aafef610bc0f193be89b80b7eb7a2316d38a5c07c15f282123b1bf3
bytes as they are: not equal
name mapped: 2 place(s) in pb-answers/a-s8r-B-payable.xml
EQUAL
exit 0
$ td-diff.py --map a-s8r-M-marks.xml pb-answers/a-s8r-M-marks.xml
a-s8r-M-marks.xml: 3398 B sha256 24c915d10371ba90319bb238ece00778da068326191e81b4c04d6ac59c5379a6
pb-answers/a-s8r-M-marks.xml: 3398 B sha256 24c915d10371ba90319bb238ece00778da068326191e81b4c04d6ac59c5379a6
bytes as they are: equal
name mapped: 0 place(s) in pb-answers/a-s8r-M-marks.xml
EQUAL
exit 0
$ td-diff.py --map a-s8r-R-CN2.xml pb-answers/a-s6r-R-CN2.xml
a-s8r-R-CN2.xml: 95828 B sha256 ddb49e859853107e464637b3805ad278227747e6682b156ce5c2ac5df5e60905
pb-answers/a-s6r-R-CN2.xml: 95830 B sha256 eb470e67de44a4aa5afd63fa5bbf3e236c2c150ce7ffb9bf66f7a554259a2ae0
bytes as they are: not equal
name mapped: 2 place(s) in pb-answers/a-s6r-R-CN2.xml
DIFFER: 10 line(s) differ
  --- pb-answers/a-s6r-R-CN2.xml
  +++ a-s8r-R-CN2.xml
  @@ -11 +11 @@
  -    <LEDGER>45</LEDGER>
  +    <LEDGER>67</LEDGER>
  @@ -19 +19 @@
  -    <CURRENCY>4</CURRENCY>
  +    <CURRENCY>6</CURRENCY>
  @@ -39 +39 @@
  -    <TAXUNIT>4</TAXUNIT>
  +    <TAXUNIT>6</TAXUNIT>
  @@ -42,2 +42,2 @@
  -    <VOUCHERNUMBERSERIES>4</VOUCHERNUMBERSERIES>
  -    <VOUCHER>8</VOUCHER>
  +    <VOUCHERNUMBERSERIES>6</VOUCHERNUMBERSERIES>
  +    <VOUCHER>12</VOUCHER>
exit 0
$ td-diff.py --map a-s8r-R-CN3.xml pb-answers/a-s7r-R-CN3.xml
a-s8r-R-CN3.xml: 95832 B sha256 f9b8fa25c51edbfffb4339cf0d28ede368ef4ddeb6d46b0778e8f9be3277fa73
pb-answers/a-s7r-R-CN3.xml: 95836 B sha256 547e5dc8e82bee41b85142b00d4e66a0ea070732d1cdc6d2a498a1a6d8bdd8c5
bytes as they are: not equal
name mapped: 2 place(s) in pb-answers/a-s7r-R-CN3.xml
DIFFER: 10 line(s) differ
  --- pb-answers/a-s7r-R-CN3.xml
  +++ a-s8r-R-CN3.xml
  @@ -11 +11 @@
  -    <LEDGER>56</LEDGER>
  +    <LEDGER>67</LEDGER>
  @@ -19 +19 @@
  -    <CURRENCY>5</CURRENCY>
  +    <CURRENCY>6</CURRENCY>
  @@ -39 +39 @@
  -    <TAXUNIT>5</TAXUNIT>
  +    <TAXUNIT>6</TAXUNIT>
  @@ -42,2 +42,2 @@
  -    <VOUCHERNUMBERSERIES>5</VOUCHERNUMBERSERIES>
  -    <VOUCHER>10</VOUCHER>
  +    <VOUCHERNUMBERSERIES>6</VOUCHERNUMBERSERIES>
  +    <VOUCHER>12</VOUCHER>
exit 0
$ td-diff.py --map a-s8r-R-0101.xml pb-answers/a-s6r-R-0101.xml
a-s8r-R-0101.xml: 95832 B sha256 9b99928875580b3f6f89ca359c65c5e225b1efd01cc67430adee2cf49d4d1c8d
pb-answers/a-s6r-R-0101.xml: 95834 B sha256 6b044785bf191d462b5c15f6131cc238c49e80a90289aac260b00ec992625b05
bytes as they are: not equal
name mapped: 2 place(s) in pb-answers/a-s6r-R-0101.xml
DIFFER: 10 line(s) differ
  --- pb-answers/a-s6r-R-0101.xml
  +++ a-s8r-R-0101.xml
  @@ -11 +11 @@
  -    <LEDGER>45</LEDGER>
  +    <LEDGER>67</LEDGER>
  @@ -19 +19 @@
  -    <CURRENCY>4</CURRENCY>
  +    <CURRENCY>6</CURRENCY>
  @@ -39 +39 @@
  -    <TAXUNIT>4</TAXUNIT>
  +    <TAXUNIT>6</TAXUNIT>
  @@ -42,2 +42,2 @@
  -    <VOUCHERNUMBERSERIES>4</VOUCHERNUMBERSERIES>
  -    <VOUCHER>8</VOUCHER>
  +    <VOUCHERNUMBERSERIES>6</VOUCHERNUMBERSERIES>
  +    <VOUCHER>12</VOUCHER>
exit 0
```

### the control: br-td.sh console
```
S-snap-20260401-20260801.xml: 26720 bytes
a-c1sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walkin BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '1576.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 1576.50
exit 0
S-snap-20260401-20260801.xml: 26720 bytes
a-c1sb-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walkin BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '1576.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 1576.50
exit 0
c1sa and c1sb byte-equal
L-lo-wk-20260801.xml: 3720 bytes
printwindow True saved <LAB_DIR>/shots/c1la-after.png
2026-11-10T07:07:36.114Z
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['3720']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0098' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: 1576.50
ANSWER: ROWS 3 (0 On Account)
exit 0
L-lo-wk-20260801.xml: 3720 bytes
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['3720']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0098' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: 1576.50
ANSWER: ROWS 3 (0 On Account)
exit 0
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26720']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26720']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['3720']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['3720']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  BILLREF 'BP/26-27/0098' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: 1576.50
ANSWER: ROWS 3 (0 On Account)
a-c1sa-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walkin BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '1576.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 1576.50
OUTCOME: told (ROWS 3 (0 On Account), sum of BILLCL 1576.50 equals closing 1576.50)
exit 0
```

### the control: td-diff.py --map against PB's s8 snapshot and report
```
$ td-diff.py --map a-c1sa-S-snap-20260401-20260801.xml pb-answers/a-s8sa-S-snap-20260401-20260801.xml
a-c1sa-S-snap-20260401-20260801.xml: 26720 B sha256 fcc1efc205c6a7018762f5988bd53f43ab5ff9c1c1073bd91eb73e34c0b5feff
pb-answers/a-s8sa-S-snap-20260401-20260801.xml: 26724 B sha256 a20c20f0e8bfc8e0180b844f29f73fcc01659b5b12850875fe0ddcea16b3a07e
bytes as they are: not equal
name mapped: 2 place(s) in pb-answers/a-s8sa-S-snap-20260401-20260801.xml
EQUAL
exit 0
$ td-diff.py --map a-c1la-L-lo-wk-20260801.xml pb-answers/a-s8la-L-lo-bc-20260801.xml
a-c1la-L-lo-wk-20260801.xml: 3720 B sha256 113fe494a6034279763f1e2d5fc14dd0e1a31465db206f6f4963993ea8938a13
pb-answers/a-s8la-L-lo-bc-20260801.xml: 3720 B sha256 113fe494a6034279763f1e2d5fc14dd0e1a31465db206f6f4963993ea8938a13
bytes as they are: equal
name mapped: 0 place(s) in pb-answers/a-s8la-L-lo-bc-20260801.xml
EQUAL
exit 0
```

### the case: br-td.sh console
```
S-snap-20260401-20270331.xml: 26762 bytes
printwindow True saved <LAB_DIR>/shots/t1sa-after.png
2026-11-10T07:08:02.737Z
a-t1sa-S-snap-20260401-20270331.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walkin BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '1576.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 1576.50
exit 0
S-snap-20260401-20270331.xml: 26762 bytes
a-t1sb-S-snap-20260401-20270331.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walkin BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '1576.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 1576.50
exit 0
t1sa and t1sb byte-equal
L-lo-wk-20270331.xml: 3732 bytes
printwindow True saved <LAB_DIR>/shots/t1la-after.png
2026-11-10T07:08:11.580Z
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['3732']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '242'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '242'
  BILLREF 'BP/26-27/0098' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '242'
  sum of BILLCL: 1576.50
ANSWER: ROWS 3 (0 On Account)
exit 0
L-lo-wk-20270331.xml: 3732 bytes
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['3732']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '242'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '242'
  BILLREF 'BP/26-27/0098' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '242'
  sum of BILLCL: 1576.50
ANSWER: ROWS 3 (0 On Account)
exit 0
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26762']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['26762']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['3732']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['3732']
  BILLREF 'BP/26-27/0099' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '242'
  BILLREF 'BP/26-27/0101' BILLDATE '1-Aug-26' BILLCL '-525.50' BILLDUE '1-Aug-26' BILLOVERDUE '242'
  BILLREF 'BP/26-27/0098' BILLDATE '1-Aug-26' BILLCL '1051.00' BILLDUE '1-Aug-26' BILLOVERDUE '242'
  sum of BILLCL: 1576.50
ANSWER: ROWS 3 (0 On Account)
a-t1sa-S-snap-20260401-20270331.xml: HEADER/STATUS 1
  LEDGER rows: 19
  'BRIDGE Walkin BC': PARENT 'Sundry Debtors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '1576.50'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 1576.50
OUTCOME: told (ROWS 3 (0 On Account), sum of BILLCL 1576.50 equals closing 1576.50)
exit 0
```

### the case: td-diff.py --same against the control
```
$ td-diff.py --same a-t1sa-S-snap-20260401-20270331.xml a-c1sa-S-snap-20260401-20260801.xml
a-t1sa-S-snap-20260401-20270331.xml: 26762 B sha256 0ef52f11c99a9119d7be8146acbb5904e78c1fb297c2708008829d0d8c6189ec
a-c1sa-S-snap-20260401-20260801.xml: 26720 B sha256 fcc1efc205c6a7018762f5988bd53f43ab5ff9c1c1073bd91eb73e34c0b5feff
bytes as they are: not equal
DIFFER: 26 line(s) differ
  --- a-c1sa-S-snap-20260401-20260801.xml
  +++ a-t1sa-S-snap-20260401-20270331.xml
  @@ -52 +52 @@
  -     <CLOSINGBALANCE TYPE="Amount">227.66</CLOSINGBALANCE>
  +     <CLOSINGBALANCE TYPE="Amount">317.81</CLOSINGBALANCE>
  @@ -108 +108 @@
  -     <CLOSINGBALANCE TYPE="Amount">0.00</CLOSINGBALANCE>
  +     <CLOSINGBALANCE TYPE="Amount">-2000.00</CLOSINGBALANCE>
  @@ -136 +136 @@
  -     <CLOSINGBALANCE TYPE="Amount">227.64</CLOSINGBALANCE>
  +     <CLOSINGBALANCE TYPE="Amount">317.77</CLOSINGBALANCE>
  @@ -164 +164 @@
  -     <CLOSINGBALANCE TYPE="Amount">0.00</CLOSINGBALANCE>
  +     <CLOSINGBALANCE TYPE="Amount">2100.00</CLOSINGBALANCE>
  @@ -178 +178 @@
  -     <CLOSINGBALANCE TYPE="Amount">0.00</CLOSINGBALANCE>
  +     <CLOSINGBALANCE TYPE="Amount">100.00</CLOSINGBALANCE>
  @@ -192 +192 @@
  -     <CLOSINGBALANCE TYPE="Amount">8505.80</CLOSINGBALANCE>
  +     <CLOSINGBALANCE TYPE="Amount">12509.60</CLOSINGBALANCE>
  @@ -206 +206 @@
  -     <CLOSINGBALANCE TYPE="Amount">599.80</CLOSINGBALANCE>
  +     <CLOSINGBALANCE TYPE="Amount">1101.00</CLOSINGBALANCE>
  @@ -220 +220 @@
  -     <CLOSINGBALANCE TYPE="Amount">0.00</CLOSINGBALANCE>
  +     <CLOSINGBALANCE TYPE="Amount">100.10</CLOSINGBALANCE>
  @@ -234 +234 @@
  -     <CLOSINGBALANCE TYPE="Amount">0.00</CLOSINGBALANCE>
  +     <CLOSINGBALANCE TYPE="Amount">400.00</CLOSINGBALANCE>
  @@ -248 +248 @@
  -     <CLOSINGBALANCE TYPE="Amount">0.00</CLOSINGBALANCE>
  +     <CLOSINGBALANCE TYPE="Amount">500.00</CLOSINGBALANCE>
  @@ -262 +262 @@
  -     <CLOSINGBALANCE TYPE="Amount">-11963.32</CLOSINGBALANCE>
  +     <CLOSINGBALANCE TYPE="Amount">-17148.70</CLOSINGBALANCE>
  @@ -290 +290 @@
  -     <CLOSINGBALANCE TYPE="Amount">0.00</CLOSINGBALANCE>
  +     <CLOSINGBALANCE TYPE="Amount">-700.00</CLOSINGBALANCE>
  @@ -304 +304 @@
  -     <CLOSINGBALANCE TYPE="Amount">9805.52</CLOSINGBALANCE>
  ... 1 more diff line(s) not shown
exit 0
$ td-diff.py --same a-t1la-L-lo-wk-20270331.xml a-c1la-L-lo-wk-20260801.xml
a-t1la-L-lo-wk-20270331.xml: 3732 B sha256 bad0c1a4a26bfe7d25b99f52c14d069833b5ee0c59ac08a7975dbdfde5dbe5ed
a-c1la-L-lo-wk-20260801.xml: 3720 B sha256 113fe494a6034279763f1e2d5fc14dd0e1a31465db206f6f4963993ea8938a13
bytes as they are: not equal
DIFFER: 6 line(s) differ
  --- a-c1la-L-lo-wk-20260801.xml
  +++ a-t1la-L-lo-wk-20270331.xml
  @@ -9 +9 @@
  - <BILLOVERDUE>0</BILLOVERDUE>
  + <BILLOVERDUE>242</BILLOVERDUE>
  @@ -32 +32 @@
  - <BILLOVERDUE>0</BILLOVERDUE>
  + <BILLOVERDUE>242</BILLOVERDUE>
  @@ -50 +50 @@
  - <BILLOVERDUE>0</BILLOVERDUE>
  + <BILLOVERDUE>242</BILLOVERDUE>
exit 0
```

### the case: the snapshot ledgers whose closing differs from the control's (a list beyond the sheet's)
```
$ the LEDGER rows of c1sa (to 20260801) and t1sa (to 20270331): each ledger whose CLOSINGBALANCE differs, and the customer's row
rows: c1sa=19, t1sa=19; same names: True
differs: BRIDGE CGST 2.5%: 227.66 -> 317.81
differs: BRIDGE Purchase Svc 5%: 0.00 -> -2000.00
differs: BRIDGE SGST 2.5%: 227.64 -> 317.77
differs: BRIDGE Supplier RJ: 0.00 -> 2100.00
differs: BRIDGE Svc 4D 5%: 0.00 -> 100.00
differs: BRIDGE Svc 998313 5%: 8505.80 -> 12509.60
differs: BRIDGE Svc 998314 5%: 599.80 -> 1101.00
differs: BRIDGE Svc 998315 5%: 0.00 -> 100.10
differs: BRIDGE Svc INH 5%: 0.00 -> 500.00
differs: BRIDGE Svc INH2 5%: 0.00 -> 400.00
differs: BRIDGE Walk-in: -11963.32 -> -17148.70
differs: Cash: 0.00 -> -700.00
differs: Profit &amp; Loss A/c: 9805.52 -> 13410.62
BRIDGE Walkin BC: 1576.50 -> 1576.50 (same)
differing: 13
```

### m1: M against s8r (--same-row)
```
== a-m1-M-marks.xml against a-s8r-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'246'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'74'", "'246'")
  equal
exit 0
```

### s9: send.sh console
```
N-0099.xml: 3022 bytes
N-0101.xml: 3022 bytes
R-0099.xml: 3022 bytes
R-0101.xml: 3022 bytes
R-CN1.xml: 3022 bytes
R-CN2.xml: 3022 bytes
R-CN3.xml: 3022 bytes
R-CN4.xml: 3022 bytes
B-bills.xml: 46 bytes
B-payable.xml: 46 bytes
S-snap-20260401-20260801.xml: 25480 bytes
S-snap-20260401-20270331.xml: 25518 bytes
M-marks.xml: 3394 bytes
a-s9-S-snap-20260401-20260801.xml: HEADER/STATUS 1
  LEDGER rows: 18
  'BRIDGE Walkin BC': no row
a-s9-S-snap-20260401-20270331.xml: HEADER/STATUS 1
  LEDGER rows: 18
  'BRIDGE Walkin BC': no row
exit 0
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
  groups for 'BRIDGE Walkin BC': 0
== a-s9-B-payable.xml: 46 bytes
  STATUS <absent>
  BILLFIXED groups: 0
  groups for 'BRIDGE Walkin BC': 0
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
$ sha256 of each s9 answer against s2's answer of the same request
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
S-snap-20260401-20270331.xml: s2 63b3a2573a91ff131c00f8e296a0aae6d6e3a02a4b150b404364d267ac0897fa, s9 63b3a2573a91ff131c00f8e296a0aae6d6e3a02a4b150b404364d267ac0897fa: equal
M-marks.xml: s2 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6, s9 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6: equal
```

### step 1: the book's data folder and the excluded GUID
```
BRIDGE PILOT LAB: folder <N>, GUID hits 2, excluded GUID hits 0
folders named so: 1
```

### each send's time to answer, and the gap before it
```
$ each send's start, its time to answer (all its files, one run.py call) and the gap from the previous send's answer to its start, seconds, from evidence/sends-utc.txt
s1: started 07:02:07.091Z; 1 file(s) answered in 1.0 s
s2: started 07:04:05.466Z; gap after s1 answered: 117.4 s; 13 file(s) answered in 2.2 s
s3m: started 07:04:23.232Z; gap after s2 answered: 15.5 s; 1 file(s) answered in 0.5 s
s3: started 07:04:25.284Z; gap after s3m answered: 1.6 s; 1 file(s) answered in 3.9 s
s4m: started 07:04:40.926Z; gap after s3 answered: 11.7 s; 1 file(s) answered in 0.4 s
s4: started 07:04:43.028Z; gap after s4m answered: 1.7 s; 1 file(s) answered in 5.4 s
s4bm: started 07:04:59.976Z; gap after s4 answered: 11.6 s; 1 file(s) answered in 0.5 s
s4b: started 07:05:02.372Z; gap after s4bm answered: 1.9 s; 1 file(s) answered in 5.0 s
s5m: started 07:05:18.875Z; gap after s4b answered: 11.5 s; 1 file(s) answered in 0.5 s
s5: started 07:05:24.292Z; gap after s5m answered: 4.9 s; 1 file(s) answered in 5.8 s
s6m: started 07:05:41.992Z; gap after s5 answered: 11.9 s; 1 file(s) answered in 0.5 s
s6: started 07:05:45.755Z; gap after s6m answered: 3.3 s; 1 file(s) answered in 5.3 s
s7m: started 07:06:03.430Z; gap after s6 answered: 12.4 s; 1 file(s) answered in 0.6 s
s7: started 07:06:06.818Z; gap after s7m answered: 2.8 s; 1 file(s) answered in 5.2 s
s8m: started 07:06:24.543Z; gap after s7 answered: 12.6 s; 1 file(s) answered in 0.5 s
s8: started 07:06:27.768Z; gap after s8m answered: 2.7 s; 1 file(s) answered in 4.6 s
s8r: started 07:06:49.146Z; gap after s8 answered: 16.8 s; 9 file(s) answered in 2.1 s
c1sa: started 07:07:26.656Z; gap after s8r answered: 35.4 s; 1 file(s) answered in 0.7 s
c1sb: started 07:07:28.680Z; gap after c1sa answered: 1.3 s; 1 file(s) answered in 0.8 s
c1la: started 07:07:31.437Z; gap after c1sb answered: 1.9 s; 1 file(s) answered in 4.8 s
c1lb: started 07:07:37.621Z; gap after c1la answered: 1.4 s; 1 file(s) answered in 0.7 s
t1sa: started 07:07:57.583Z; gap after c1lb answered: 19.3 s; 1 file(s) answered in 5.3 s
t1sb: started 07:08:04.213Z; gap after t1sa answered: 1.4 s; 1 file(s) answered in 0.8 s
t1la: started 07:08:06.867Z; gap after t1sb answered: 1.9 s; 1 file(s) answered in 4.9 s
t1lb: started 07:08:13.116Z; gap after t1la answered: 1.4 s; 1 file(s) answered in 0.8 s
m1: started 07:08:49.751Z; gap after t1lb answered: 35.8 s; 1 file(s) answered in 0.7 s
s9: started 07:11:51.004Z; gap after m1 answered: 180.5 s; 13 file(s) answered in 1.7 s

$ each send's time from its start to its last answer file's write time (file mtime), so without the window capture that send.sh takes after an import or a named shot
s1: the last answer file written 0.8 s after the send began (1 file(s))
s2: the last answer file written 2.1 s after the send began (13 file(s))
s3m: the last answer file written 0.4 s after the send began (1 file(s))
s3: the last answer file written 0.7 s after the send began (1 file(s))
s4m: the last answer file written 0.3 s after the send began (1 file(s))
s4: the last answer file written 1.9 s after the send began (1 file(s))
s4bm: the last answer file written 0.4 s after the send began (1 file(s))
s4b: the last answer file written 1.8 s after the send began (1 file(s))
s5m: the last answer file written 0.4 s after the send began (1 file(s))
s5: the last answer file written 1.6 s after the send began (1 file(s))
s6m: the last answer file written 0.4 s after the send began (1 file(s))
s6: the last answer file written 1.4 s after the send began (1 file(s))
s7m: the last answer file written 0.5 s after the send began (1 file(s))
s7: the last answer file written 1.1 s after the send began (1 file(s))
s8m: the last answer file written 0.4 s after the send began (1 file(s))
s8: the last answer file written 1.1 s after the send began (1 file(s))
s8r: the last answer file written 2.0 s after the send began (9 file(s))
c1sa: the last answer file written 0.5 s after the send began (1 file(s))
c1sb: the last answer file written 0.7 s after the send began (1 file(s))
c1la: the last answer file written 0.6 s after the send began (1 file(s))
c1lb: the last answer file written 0.6 s after the send began (1 file(s))
t1sa: the last answer file written 0.6 s after the send began (1 file(s))
t1sb: the last answer file written 0.5 s after the send began (1 file(s))
t1la: the last answer file written 0.6 s after the send began (1 file(s))
t1lb: the last answer file written 0.6 s after the send began (1 file(s))
m1: the last answer file written 0.5 s after the send began (1 file(s))
s9: the last answer file written 1.6 s after the send began (13 file(s))
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
a-s2-S-snap-20260401-20270331.xml: all 0 (names in all: 35)
a-s2-M-marks.xml: all 0 (names in all: 35)
a-s3m-M-marks.xml: all 0 (names in all: 35)
a-s4m-M-marks.xml: LEDGER 1; others 0 (names in all: 35)
a-s4bm-M-marks.xml: LEDGER 12; CURRENCY 1; TAXUNIT 1; VOUCHERNUMBERSERIES 1; VOUCHER 2; others 0 (names in all: 35)
a-s5m-M-marks.xml: LEDGER 23; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 4; others 0 (names in all: 35)
a-s6m-M-marks.xml: LEDGER 34; CURRENCY 3; TAXUNIT 3; VOUCHERNUMBERSERIES 3; VOUCHER 6; others 0 (names in all: 35)
a-s7m-M-marks.xml: LEDGER 45; CURRENCY 4; TAXUNIT 4; VOUCHERNUMBERSERIES 4; VOUCHER 8; others 0 (names in all: 35)
a-s8m-M-marks.xml: LEDGER 56; CURRENCY 5; TAXUNIT 5; VOUCHERNUMBERSERIES 5; VOUCHER 10; others 0 (names in all: 35)
a-s8r-R-CN4.xml: LEDGER 67; CURRENCY 6; TAXUNIT 6; VOUCHERNUMBERSERIES 6; VOUCHER 12; others 0 (names in all: 35)
a-s8r-R-CN1.xml: LEDGER 67; CURRENCY 6; TAXUNIT 6; VOUCHERNUMBERSERIES 6; VOUCHER 12; others 0 (names in all: 35)
a-s8r-R-CN2.xml: LEDGER 67; CURRENCY 6; TAXUNIT 6; VOUCHERNUMBERSERIES 6; VOUCHER 12; others 0 (names in all: 35)
a-s8r-R-CN3.xml: LEDGER 67; CURRENCY 6; TAXUNIT 6; VOUCHERNUMBERSERIES 6; VOUCHER 12; others 0 (names in all: 35)
a-s8r-R-0099.xml: LEDGER 67; CURRENCY 6; TAXUNIT 6; VOUCHERNUMBERSERIES 6; VOUCHER 12; others 0 (names in all: 35)
a-s8r-R-0101.xml: LEDGER 67; CURRENCY 6; TAXUNIT 6; VOUCHERNUMBERSERIES 6; VOUCHER 12; others 0 (names in all: 35)
a-s8r-M-marks.xml: LEDGER 67; CURRENCY 6; TAXUNIT 6; VOUCHERNUMBERSERIES 6; VOUCHER 12; others 0 (names in all: 35)
a-c1sa-S-snap-20260401-20260801.xml: LEDGER 67; CURRENCY 6; TAXUNIT 6; VOUCHERNUMBERSERIES 6; VOUCHER 12; others 0 (names in all: 35)
a-c1sb-S-snap-20260401-20260801.xml: LEDGER 67; CURRENCY 6; TAXUNIT 6; VOUCHERNUMBERSERIES 6; VOUCHER 12; others 0 (names in all: 35)
a-t1sa-S-snap-20260401-20270331.xml: LEDGER 67; CURRENCY 6; TAXUNIT 6; VOUCHERNUMBERSERIES 6; VOUCHER 12; others 0 (names in all: 35)
a-t1sb-S-snap-20260401-20270331.xml: LEDGER 67; CURRENCY 6; TAXUNIT 6; VOUCHERNUMBERSERIES 6; VOUCHER 12; others 0 (names in all: 35)
a-m1-M-marks.xml: LEDGER 67; CURRENCY 6; TAXUNIT 6; VOUCHERNUMBERSERIES 6; VOUCHER 12; others 0 (names in all: 35)
a-s9-N-0099.xml: all 0 (names in all: 35)
a-s9-N-0101.xml: all 0 (names in all: 35)
a-s9-R-0099.xml: all 0 (names in all: 35)
a-s9-R-0101.xml: all 0 (names in all: 35)
a-s9-R-CN1.xml: all 0 (names in all: 35)
a-s9-R-CN2.xml: all 0 (names in all: 35)
a-s9-R-CN3.xml: all 0 (names in all: 35)
a-s9-R-CN4.xml: all 0 (names in all: 35)
a-s9-S-snap-20260401-20260801.xml: all 0 (names in all: 35)
a-s9-S-snap-20260401-20270331.xml: all 0 (names in all: 35)
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
a-s2-S-snap-20260401-20270331.xml 25518 B 63b3a2573a91ff131c00f8e296a0aae6d6e3a02a4b150b404364d267ac0897fa
a-s2-S-snap-20260401-20270331.xml.headers 135 B afb8084022c5eeb00cda14288a1d340534e0c41bf587553aab4df0e69298d93e
a-s2-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s2-M-marks.xml.headers 134 B a8c6fe9f6545175fb5e08b9ee53ba543601c0436c80c231f7b009d73e5c8ccf0
a-s3m-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s3m-M-marks.xml.headers 134 B a8c6fe9f6545175fb5e08b9ee53ba543601c0436c80c231f7b009d73e5c8ccf0
a-s3-L-ledger.xml 538 B 1b72152a8fd20ccb9366277864bf32357b8891e536cde2a54992adaa60a97010
a-s3-L-ledger.xml.headers 133 B 19b28d1b18183bea529f520e0ce9251ee2655e6b58302a8b47562695a42a91b7
a-s4m-M-marks.xml 3394 B 49bdfb3f21e1db5f7d0c4991efa5683bbb0d9cb180ad8092b961024713ac66a7
a-s4m-M-marks.xml.headers 134 B a8c6fe9f6545175fb5e08b9ee53ba543601c0436c80c231f7b009d73e5c8ccf0
a-s4-I-0099.xml 540 B fdcd8435c6fcf95ab624cbe2cff2c887078df500626f7c9c37cfa834a1cdb6ea
a-s4-I-0099.xml.headers 133 B 2a0eba722d654c2b2fa59d71608c67e3849d25c56207357fa06b0239977a4670
a-s4bm-M-marks.xml 3396 B 17c5bde53bf5e459f4ae36fce8ee62e8bfe5eb636f47f1970faff44c230a59ed
a-s4bm-M-marks.xml.headers 134 B db9b137db28569a10d72e1f9edb1f97fa4dc3c0f0c092ea2d8b3e1ba4cae016c
a-s4b-I-0101.xml 540 B 32e7a29b201c773547aa47bc95bd3eda8af83aeb6e35090303b9b27bcd34eec5
a-s4b-I-0101.xml.headers 133 B 2a0eba722d654c2b2fa59d71608c67e3849d25c56207357fa06b0239977a4670
a-s5m-M-marks.xml 3396 B d55cf90ef81a5c355be9e694f8fae317da2e1701fa95feb6f4cc446cb4a7553e
a-s5m-M-marks.xml.headers 134 B db9b137db28569a10d72e1f9edb1f97fa4dc3c0f0c092ea2d8b3e1ba4cae016c
a-s5-I-CN-1.xml 540 B f8384a874e1d8a6be92c2969f94e2887fb700768f6e111b6b3f649cd13ffdc7e
a-s5-I-CN-1.xml.headers 133 B 2a0eba722d654c2b2fa59d71608c67e3849d25c56207357fa06b0239977a4670
a-s6m-M-marks.xml 3396 B 4ac186ec14f33c8066791cc711c6b86445781db3758c5abc3d982280fa4b5b7f
a-s6m-M-marks.xml.headers 134 B db9b137db28569a10d72e1f9edb1f97fa4dc3c0f0c092ea2d8b3e1ba4cae016c
a-s6-I-CN-P.xml 540 B e6540f6534095f5608f3c59f802c1f40a3ae13529a7e9041847b460f03c32fec
a-s6-I-CN-P.xml.headers 133 B 2a0eba722d654c2b2fa59d71608c67e3849d25c56207357fa06b0239977a4670
a-s7m-M-marks.xml 3396 B 419f6b56d7e3f2add5ccb53dede6f88457ffc8a20399d3d60a4786a0d48e062e
a-s7m-M-marks.xml.headers 134 B db9b137db28569a10d72e1f9edb1f97fa4dc3c0f0c092ea2d8b3e1ba4cae016c
a-s7-I-CN-2.xml 540 B 93e331323bb1235565ab31f2ddbbaf7afaa752ce6770d0ae7ae145d7b3244234
a-s7-I-CN-2.xml.headers 133 B 2a0eba722d654c2b2fa59d71608c67e3849d25c56207357fa06b0239977a4670
a-s8m-M-marks.xml 3398 B 854002ea209a04d49b046a679264a052652a8f3b837c36c926ef7b7e0b1e84b1
a-s8m-M-marks.xml.headers 134 B 4d2c776864af2995685156531c0e1bd6dfa88116418863514e54213f2a5ee6a0
a-s8-I-CN-3.xml 540 B 200c17e120c3ff582d28c9fce94b127ed22aaa4e5d10bbdd1d18e97a767f40c3
a-s8-I-CN-3.xml.headers 133 B 2a0eba722d654c2b2fa59d71608c67e3849d25c56207357fa06b0239977a4670
a-s8r-R-CN4.xml 95832 B 90b932dc8003ab78509567410dbb41031d30de28272704028cfca705ce36a264
a-s8r-R-CN4.xml.headers 135 B e75b133421d794e079e97f488c356b9ccc0b7e8bc7b0d172a6e4a68ac826e541
a-s8r-R-CN1.xml 95832 B 1ddb030cd305b29106759c8dd41ff9433445eb6996de99537598387e720aa3b1
a-s8r-R-CN1.xml.headers 135 B e75b133421d794e079e97f488c356b9ccc0b7e8bc7b0d172a6e4a68ac826e541
a-s8r-R-CN2.xml 95828 B ddb49e859853107e464637b3805ad278227747e6682b156ce5c2ac5df5e60905
a-s8r-R-CN2.xml.headers 135 B c0f257411bc5a0e010c6112aee9c2ce89e25aba32ba1a5da76efbbda215a8a7e
a-s8r-R-CN3.xml 95832 B f9b8fa25c51edbfffb4339cf0d28ede368ef4ddeb6d46b0778e8f9be3277fa73
a-s8r-R-CN3.xml.headers 135 B e75b133421d794e079e97f488c356b9ccc0b7e8bc7b0d172a6e4a68ac826e541
a-s8r-R-0099.xml 95832 B b5af11b23955c6cf9142d97afa3283b89fc1e9a963a9374b71b0fafa98a13ca4
a-s8r-R-0099.xml.headers 135 B e75b133421d794e079e97f488c356b9ccc0b7e8bc7b0d172a6e4a68ac826e541
a-s8r-R-0101.xml 95832 B 9b99928875580b3f6f89ca359c65c5e225b1efd01cc67430adee2cf49d4d1c8d
a-s8r-R-0101.xml.headers 135 B e75b133421d794e079e97f488c356b9ccc0b7e8bc7b0d172a6e4a68ac826e541
a-s8r-B-bills.xml 506 B a43ee72c3ffb4a03fea640e6ba4284f8c8197e13de2fd1513f834305b95fd5af
a-s8r-B-bills.xml.headers 133 B 9d806f86bc00fe7c9a143ad6d534f2c96a06fe5bcd5bb64de0c8238a5aff5a95
a-s8r-B-payable.xml 962 B 96b5e3c7c5fc8c9d75443684cad5ad44dd419041795c59c622085780a70de90e
a-s8r-B-payable.xml.headers 133 B 0496c762ce6e90a3117dfbde3df88f799456cf0385b354d0d8fc75cc32caccb5
a-s8r-M-marks.xml 3398 B 24c915d10371ba90319bb238ece00778da068326191e81b4c04d6ac59c5379a6
a-s8r-M-marks.xml.headers 134 B 4d2c776864af2995685156531c0e1bd6dfa88116418863514e54213f2a5ee6a0
a-c1sa-S-snap-20260401-20260801.xml 26720 B fcc1efc205c6a7018762f5988bd53f43ab5ff9c1c1073bd91eb73e34c0b5feff
a-c1sa-S-snap-20260401-20260801.xml.headers 135 B b4cbabd88b181e53684aa25394dbf80e5ab612013f1a06f82dce88a675dfe4a3
a-c1sb-S-snap-20260401-20260801.xml 26720 B fcc1efc205c6a7018762f5988bd53f43ab5ff9c1c1073bd91eb73e34c0b5feff
a-c1sb-S-snap-20260401-20260801.xml.headers 135 B b4cbabd88b181e53684aa25394dbf80e5ab612013f1a06f82dce88a675dfe4a3
a-c1la-L-lo-wk-20260801.xml 3720 B 113fe494a6034279763f1e2d5fc14dd0e1a31465db206f6f4963993ea8938a13
a-c1la-L-lo-wk-20260801.xml.headers 134 B fc1252100299388875c14f55399a5c7ef8d50c25cd9dc253fdff0e4ccf6121e8
a-c1lb-L-lo-wk-20260801.xml 3720 B 113fe494a6034279763f1e2d5fc14dd0e1a31465db206f6f4963993ea8938a13
a-c1lb-L-lo-wk-20260801.xml.headers 134 B fc1252100299388875c14f55399a5c7ef8d50c25cd9dc253fdff0e4ccf6121e8
a-t1sa-S-snap-20260401-20270331.xml 26762 B 0ef52f11c99a9119d7be8146acbb5904e78c1fb297c2708008829d0d8c6189ec
a-t1sa-S-snap-20260401-20270331.xml.headers 135 B 6b51411f1a461d86f6721cb372256f389038d5e894f1f7f1fa4acff121383d16
a-t1sb-S-snap-20260401-20270331.xml 26762 B 0ef52f11c99a9119d7be8146acbb5904e78c1fb297c2708008829d0d8c6189ec
a-t1sb-S-snap-20260401-20270331.xml.headers 135 B 6b51411f1a461d86f6721cb372256f389038d5e894f1f7f1fa4acff121383d16
a-t1la-L-lo-wk-20270331.xml 3732 B bad0c1a4a26bfe7d25b99f52c14d069833b5ee0c59ac08a7975dbdfde5dbe5ed
a-t1la-L-lo-wk-20270331.xml.headers 134 B 9eb24d23d5dae26c7a632bd754468db0f403aca41f7947a3f595fb5c054a05c9
a-t1lb-L-lo-wk-20270331.xml 3732 B bad0c1a4a26bfe7d25b99f52c14d069833b5ee0c59ac08a7975dbdfde5dbe5ed
a-t1lb-L-lo-wk-20270331.xml.headers 134 B 9eb24d23d5dae26c7a632bd754468db0f403aca41f7947a3f595fb5c054a05c9
a-m1-M-marks.xml 3398 B 24c915d10371ba90319bb238ece00778da068326191e81b4c04d6ac59c5379a6
a-m1-M-marks.xml.headers 134 B 4d2c776864af2995685156531c0e1bd6dfa88116418863514e54213f2a5ee6a0
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
a-s9-S-snap-20260401-20270331.xml 25518 B 63b3a2573a91ff131c00f8e296a0aae6d6e3a02a4b150b404364d267ac0897fa
a-s9-S-snap-20260401-20270331.xml.headers 135 B afb8084022c5eeb00cda14288a1d340534e0c41bf587553aab4df0e69298d93e
a-s9-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s9-M-marks.xml.headers 134 B a8c6fe9f6545175fb5e08b9ee53ba543601c0436c80c231f7b009d73e5c8ccf0
```

### each send's start and answer, UTC
```
2026-11-10T07:02:07.091Z s1 M-marks.xml (hashes equal to the list)
2026-11-10T07:02:08.068Z s1 answered
2026-11-10T07:04:05.466Z s2 N-0099.xml N-0101.xml R-0099.xml R-0101.xml R-CN1.xml R-CN2.xml R-CN3.xml R-CN4.xml B-bills.xml B-payable.xml S-snap-20260401-20260801.xml S-snap-20260401-20270331.xml M-marks.xml (hashes equal to the list)
2026-11-10T07:04:07.692Z s2 answered
2026-11-10T07:04:23.232Z s3m M-marks.xml (hashes equal to the list)
2026-11-10T07:04:23.693Z s3m answered
2026-11-10T07:04:25.284Z s3 L-ledger.xml (hashes equal to the list)
2026-11-10T07:04:29.232Z s3 answered
2026-11-10T07:04:40.926Z s4m M-marks.xml (hashes equal to the list)
2026-11-10T07:04:41.363Z s4m answered
2026-11-10T07:04:43.028Z s4 I-0099.xml (hashes equal to the list)
2026-11-10T07:04:48.383Z s4 answered
2026-11-10T07:04:59.976Z s4bm M-marks.xml (hashes equal to the list)
2026-11-10T07:05:00.487Z s4bm answered
2026-11-10T07:05:02.372Z s4b I-0101.xml (hashes equal to the list)
2026-11-10T07:05:07.391Z s4b answered
2026-11-10T07:05:18.875Z s5m M-marks.xml (hashes equal to the list)
2026-11-10T07:05:19.354Z s5m answered
2026-11-10T07:05:24.292Z s5 I-CN-1.xml (hashes equal to the list)
2026-11-10T07:05:30.132Z s5 answered
2026-11-10T07:05:41.992Z s6m M-marks.xml (hashes equal to the list)
2026-11-10T07:05:42.499Z s6m answered
2026-11-10T07:05:45.755Z s6 I-CN-P.xml (hashes equal to the list)
2026-11-10T07:05:51.032Z s6 answered
2026-11-10T07:06:03.430Z s7m M-marks.xml (hashes equal to the list)
2026-11-10T07:06:04.020Z s7m answered
2026-11-10T07:06:06.818Z s7 I-CN-2.xml (hashes equal to the list)
2026-11-10T07:06:11.988Z s7 answered
2026-11-10T07:06:24.543Z s8m M-marks.xml (hashes equal to the list)
2026-11-10T07:06:25.044Z s8m answered
2026-11-10T07:06:27.768Z s8 I-CN-3.xml (hashes equal to the list)
2026-11-10T07:06:32.358Z s8 answered
2026-11-10T07:06:49.146Z s8r R-CN4.xml R-CN1.xml R-CN2.xml R-CN3.xml R-0099.xml R-0101.xml B-bills.xml B-payable.xml M-marks.xml (hashes equal to the list)
2026-11-10T07:06:51.288Z s8r answered
2026-11-10T07:07:26.656Z c1sa S-snap-20260401-20260801.xml (hashes equal to the list)
2026-11-10T07:07:27.359Z c1sa answered
2026-11-10T07:07:28.680Z c1sb S-snap-20260401-20260801.xml (hashes equal to the list)
2026-11-10T07:07:29.507Z c1sb answered
2026-11-10T07:07:31.437Z c1la L-lo-wk-20260801.xml (hashes equal to the list)
2026-11-10T07:07:36.240Z c1la answered
2026-11-10T07:07:37.621Z c1lb L-lo-wk-20260801.xml (hashes equal to the list)
2026-11-10T07:07:38.326Z c1lb answered
2026-11-10T07:07:57.583Z t1sa S-snap-20260401-20270331.xml (hashes equal to the list)
2026-11-10T07:08:02.842Z t1sa answered
2026-11-10T07:08:04.213Z t1sb S-snap-20260401-20270331.xml (hashes equal to the list)
2026-11-10T07:08:04.963Z t1sb answered
2026-11-10T07:08:06.867Z t1la L-lo-wk-20270331.xml (hashes equal to the list)
2026-11-10T07:08:11.753Z t1la answered
2026-11-10T07:08:13.116Z t1lb L-lo-wk-20270331.xml (hashes equal to the list)
2026-11-10T07:08:13.910Z t1lb answered
2026-11-10T07:08:49.751Z m1 M-marks.xml (hashes equal to the list)
2026-11-10T07:08:50.476Z m1 answered
2026-11-10T07:11:51.004Z s9 N-0099.xml N-0101.xml R-0099.xml R-0101.xml R-CN1.xml R-CN2.xml R-CN3.xml R-CN4.xml B-bills.xml B-payable.xml S-snap-20260401-20260801.xml S-snap-20260401-20270331.xml M-marks.xml (hashes equal to the list)
2026-11-10T07:11:52.746Z s9 answered
```

## run.py logs (each send: local IST time to the second, request, answer file, curl exit, bytes)

### s1
```
12:32:07	M-marks.xml	a-s1-M-marks.xml	curl 0	3394 bytes
```

### s2
```
12:34:05	N-0099.xml	a-s2-N-0099.xml	curl 0	3022 bytes
12:34:06	N-0101.xml	a-s2-N-0101.xml	curl 0	3022 bytes
12:34:06	R-0099.xml	a-s2-R-0099.xml	curl 0	3022 bytes
12:34:06	R-0101.xml	a-s2-R-0101.xml	curl 0	3022 bytes
12:34:06	R-CN1.xml	a-s2-R-CN1.xml	curl 0	3022 bytes
12:34:06	R-CN2.xml	a-s2-R-CN2.xml	curl 0	3022 bytes
12:34:06	R-CN3.xml	a-s2-R-CN3.xml	curl 0	3022 bytes
12:34:07	R-CN4.xml	a-s2-R-CN4.xml	curl 0	3022 bytes
12:34:07	B-bills.xml	a-s2-B-bills.xml	curl 0	46 bytes
12:34:07	B-payable.xml	a-s2-B-payable.xml	curl 0	46 bytes
12:34:07	S-snap-20260401-20260801.xml	a-s2-S-snap-20260401-20260801.xml	curl 0	25480 bytes
12:34:07	S-snap-20260401-20270331.xml	a-s2-S-snap-20260401-20270331.xml	curl 0	25518 bytes
12:34:07	M-marks.xml	a-s2-M-marks.xml	curl 0	3394 bytes
```

### s3m
```
12:34:23	M-marks.xml	a-s3m-M-marks.xml	curl 0	3394 bytes
```

### s3
```
12:34:25	L-ledger.xml	a-s3-L-ledger.xml	curl 0	538 bytes
```

### s4m
```
12:34:41	M-marks.xml	a-s4m-M-marks.xml	curl 0	3394 bytes
```

### s4
```
12:34:44	I-0099.xml	a-s4-I-0099.xml	curl 0	540 bytes
```

### s4bm
```
12:35:00	M-marks.xml	a-s4bm-M-marks.xml	curl 0	3396 bytes
```

### s4b
```
12:35:03	I-0101.xml	a-s4b-I-0101.xml	curl 0	540 bytes
```

### s5m
```
12:35:19	M-marks.xml	a-s5m-M-marks.xml	curl 0	3396 bytes
```

### s5
```
12:35:25	I-CN-1.xml	a-s5-I-CN-1.xml	curl 0	540 bytes
```

### s6m
```
12:35:42	M-marks.xml	a-s6m-M-marks.xml	curl 0	3396 bytes
```

### s6
```
12:35:46	I-CN-P.xml	a-s6-I-CN-P.xml	curl 0	540 bytes
```

### s7m
```
12:36:03	M-marks.xml	a-s7m-M-marks.xml	curl 0	3396 bytes
```

### s7
```
12:36:07	I-CN-2.xml	a-s7-I-CN-2.xml	curl 0	540 bytes
```

### s8m
```
12:36:24	M-marks.xml	a-s8m-M-marks.xml	curl 0	3398 bytes
```

### s8
```
12:36:28	I-CN-3.xml	a-s8-I-CN-3.xml	curl 0	540 bytes
```

### s8r
```
12:36:49	R-CN4.xml	a-s8r-R-CN4.xml	curl 0	95832 bytes
12:36:49	R-CN1.xml	a-s8r-R-CN1.xml	curl 0	95832 bytes
12:36:50	R-CN2.xml	a-s8r-R-CN2.xml	curl 0	95828 bytes
12:36:50	R-CN3.xml	a-s8r-R-CN3.xml	curl 0	95832 bytes
12:36:50	R-0099.xml	a-s8r-R-0099.xml	curl 0	95832 bytes
12:36:50	R-0101.xml	a-s8r-R-0101.xml	curl 0	95832 bytes
12:36:50	B-bills.xml	a-s8r-B-bills.xml	curl 0	506 bytes
12:36:51	B-payable.xml	a-s8r-B-payable.xml	curl 0	962 bytes
12:36:51	M-marks.xml	a-s8r-M-marks.xml	curl 0	3398 bytes
```

### c1sa
```
12:37:27	S-snap-20260401-20260801.xml	a-c1sa-S-snap-20260401-20260801.xml	curl 0	26720 bytes
```

### c1sb
```
12:37:29	S-snap-20260401-20260801.xml	a-c1sb-S-snap-20260401-20260801.xml	curl 0	26720 bytes
```

### c1la
```
12:37:31	L-lo-wk-20260801.xml	a-c1la-L-lo-wk-20260801.xml	curl 0	3720 bytes
```

### c1lb
```
12:37:38	L-lo-wk-20260801.xml	a-c1lb-L-lo-wk-20260801.xml	curl 0	3720 bytes
```

### t1sa
```
12:37:58	S-snap-20260401-20270331.xml	a-t1sa-S-snap-20260401-20270331.xml	curl 0	26762 bytes
```

### t1sb
```
12:38:04	S-snap-20260401-20270331.xml	a-t1sb-S-snap-20260401-20270331.xml	curl 0	26762 bytes
```

### t1la
```
12:38:07	L-lo-wk-20270331.xml	a-t1la-L-lo-wk-20270331.xml	curl 0	3732 bytes
```

### t1lb
```
12:38:13	L-lo-wk-20270331.xml	a-t1lb-L-lo-wk-20270331.xml	curl 0	3732 bytes
```

### m1
```
12:38:50	M-marks.xml	a-m1-M-marks.xml	curl 0	3398 bytes
```

### s9
```
12:41:51	N-0099.xml	a-s9-N-0099.xml	curl 0	3022 bytes
12:41:51	N-0101.xml	a-s9-N-0101.xml	curl 0	3022 bytes
12:41:51	R-0099.xml	a-s9-R-0099.xml	curl 0	3022 bytes
12:41:51	R-0101.xml	a-s9-R-0101.xml	curl 0	3022 bytes
12:41:51	R-CN1.xml	a-s9-R-CN1.xml	curl 0	3022 bytes
12:41:51	R-CN2.xml	a-s9-R-CN2.xml	curl 0	3022 bytes
12:41:52	R-CN3.xml	a-s9-R-CN3.xml	curl 0	3022 bytes
12:41:52	R-CN4.xml	a-s9-R-CN4.xml	curl 0	3022 bytes
12:41:52	B-bills.xml	a-s9-B-bills.xml	curl 0	46 bytes
12:41:52	B-payable.xml	a-s9-B-payable.xml	curl 0	46 bytes
12:41:52	S-snap-20260401-20260801.xml	a-s9-S-snap-20260401-20260801.xml	curl 0	25480 bytes
12:41:52	S-snap-20260401-20270331.xml	a-s9-S-snap-20260401-20270331.xml	curl 0	25518 bytes
12:41:52	M-marks.xml	a-s9-M-marks.xml	curl 0	3394 bytes
```
