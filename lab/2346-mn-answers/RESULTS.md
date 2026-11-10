# Row 2346-MN: keyed Manual note types kept the number sent and re-stated a same-key resend; a new key with the same number was declined in the answer; the class reads told credit from debit; but the same number was created again in a second credit type and in the debit type

Run on 10 Nov 2026, 03:46:45Z to 04:08:40Z (09:16 to 09:38 IST), on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2, as row 2162-KD read it from the program's file; the Gateway shows no release number), by the sheet at `lab/2346-mn` (cec5d5a8), on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic), loaded alone in a sitting of its own. Three voucher types keyed on Tally's screen; six imports, each file sent once, `I-MC1-again.xml` (byte-equal to `I-MC1.xml`) under the sheet's one C3 exception at step 8, on the run lead's yes when queuing the row; read-only reads between them, one at a time through `run.py`; no Bridge build. Then the restore of this sitting's own backup. Every value below is M, measured by this run, in one run. Every read was answered, each with STATUS 1. The window after each import was the Gateway, no dialog.

This sitting ran after rows 2346-AR (Tally quit at 03:25:45Z) and 683-CD (Tally quit after 03:38:23Z), with a Tally start of its own, so step 1's "otherwise" applied: step 0 of C2, then the book loaded alone.

## Result

Each is PARTIAL by the sheet's Decides: one sitting, one book, one release.

| import | step | `CREATED` | `ALTERED` | `EXCEPTIONS` | `ERRORS` | `LASTVCHID` | LINEERROR |
|---|---|---|---|---|---|---|---|
| `I-MC1.xml`, CN/26-27/0001 into `BRIDGE Credit Note Manual` | s6 | 1 | 0 | 0 | 0 | 45 | none |
| `I-MD1.xml`, DN/26-27/0001 into `BRIDGE Debit Note Manual` | s7 | 1 | 0 | 0 | 0 | 46 | none |
| `I-MC1-again.xml`, the same request again | s8 | 0 | 1 | 0 | 0 | 45 | none |
| `I-MC1n.xml`, the same number under a new key | s9 | 0 | 0 | 1 | 0 | 0 | none |
| `I-MC2.xml`, the credit number into `BRIDGE Credit Note Manual 2` | s11 | 1 | 0 | 0 | 0 | 47 | none |
| `I-MDX.xml`, the credit number into `BRIDGE Debit Note Manual` | s12 | 1 | 0 | 0 | 0 | 48 | none |

`DELETED`, `LASTMID`, `COMBINED`, `IGNORED` and `CANCELLED` were 0 in all six answers.

**Series and keying (s2 to s5r).**
- s2: both predefined note types as row 2162-KD's s2 read them (this T answer is byte-equal to that one): `Credit Note` and `Debit Note` each one series `Default`, `Automatic`, `Auto Retain`, `PREVENTDUPLICATES` No. By the Decides: "the pilot's note types are Automatic `Auto Retain` from two sittings' series reads."
- s3r, s4r and s5r: each keyed type read one series `Default`, `Manual`, `NUMBERINGSUBMETHOD` `Default`, `PREVENTDUPLICATES` Yes, its `PARENT` as keyed, `NUMBERINGMETHOD` `None`, as row 2162-KD's keyed Sales type. Each key raised ALTMSTID by 2 (245, 247, 249, 251) and left ALTVCHID at 68. By the Decides: "a note type keyed so carries the flag the build reads."
- **The GUID predictions.** `BRIDGE Credit Note Manual` took `-000000e8`, as predicted. `BRIDGE Debit Note Manual` took **`-000000ea`**, not `-000000e9`; `BRIDGE Credit Note Manual 2` took **`-000000ec`**, not `-000000ea`. Each key took every second MasterID; no voucher type holds `-000000e9`, and the string is in no T answer. By the sheet this is not a stop: `R-MD1.xml`, `R-MDX.xml` (both `-000000e9`) and `R-MC2.xml` (`-000000ea`, which the debit type holds) were not sent, and the V reads stand in their place.

**Create and read (s6, s7): "Stored as sent and found by the read-back."**
- R-MC1 (the build's read-back with the type's GUID `-000000e8`): one row, `CN/26-27/0001`, `BRIDGE Credit Note Manual`, date 20260801, GUID tail `0000002d`, ALTERID 69, the six legs as sent.
- V-CN: one row, the same number and type, MASTERID 45 (the import's `LASTVCHID`), GUID the book's GUID followed by `0000002d`.
- V-DN at s7r: one row, `DN/26-27/0001`, `BRIDGE Debit Note Manual`, MASTERID 46, GUID tail `0000002e`. The debit side's read-back (R-MD1) was not sent, so on that side the note was found by V, not by the build's read-back.

**Same-key resend (s8): "re-stated".** `CREATED` 0, `ALTERED` 1. V-CN then held one row: GUID `-0000002d` and MASTERID 45 as at s6r, ALTERID 69 -> 71. `--diff` of R-MC1, s6r against s8r: ALTERID only; legs 6 -> 6. By the Decides: "a same-key resend is safe on a note type, as §3.3a found for a voucher."

**New-key resend (s9): "declined in the answer".** `CREATED` 0, `EXCEPTIONS` 1, every other counter 0, no LINEERROR, no dialog, as row 2162-KD's s5 on Sales. V-CN as at s8r; the marks equal to s8r's. By the Decides: "the guard holds on a note type within the type, in-band."

**Class reads (s10): "as the table".**

| read | rows | class element |
|---|---|---|
| N-0010s (the build's read, the control) | 1, `BP/26-27/0010` on `BRIDGE Sales` | `BRIDGEVCHISSALES` Yes |
| N-0010c, N-0010d | the same row | `BRIDGEVCHISCREDITNOTE` No; `BRIDGEVCHISDEBITNOTE` No |
| N-CNc, N-CNd | 1, `CN/26-27/0001` on `BRIDGE Credit Note Manual` | credit Yes; debit No |
| N-DNd, N-DNc | 1, `DN/26-27/0001` on `BRIDGE Debit Note Manual` | debit Yes; credit No |

No row lacked its class element. By the Decides: "the number read tells the classes apart inside its `COMPUTE`."

**The guard across types (s11, s12).**
- s11, `CN/26-27/0001` into the second Manual credit type: **created**. V-CN then held two rows, the same number on each credit type (MASTERID 45 and 47; GUID tails `0000002d` and `0000002f`); N-CNc read both as credit Yes. R-MC1 (the first type's GUID) still found only its own note. By the Decides: "the guard is per type. Between two Manual credit types, the class read before the post is the only guard."
- s12, `CN/26-27/0001` into the Manual debit type: **created**. V-CN then held three rows; the third on `BRIDGE Debit Note Manual`, MASTERID 48, GUID tail `00000030`. N-CNc read it credit No and N-CNd debit Yes, the two credit rows the other way about. By the Decides: "credit and debit numbers are separate series in Tally, which agrees with condition 1's per-class domain."

**The way back (s13).** Restored by C6 with only the two expected prompts. M byte-equal to s1 (ALTVCHID 68, ALTMSTID 245); T 28 rows, sha256 equal to s2's, the three keyed names 0; V-CN and V-DN 0 rows, each sha256 equal to s2's. Every part of the compare was equal.

**The sheet's estimates:** s2, s3r to s5r (series), s6, s7, s9, s10 and s13 each as estimated. s8, s11 and s12 were not estimated.

**Deviations from the sheet and the lab's rules:**
1. **Bridge processes.** Three Bridge processes were running at the start. On the run lead's standing word none was ended, and they were left running at the close. No Bridge tool was called and no request went through Bridge.
2. **The start-up company.** Tally's start-up company loaded by itself, as the install is set to. It was shut by the K: Company label, Shut, Enter on it, before step 0, on the run lead's standing practice for these sittings, with no change to it.
3. **Three R reads not sent**, as the sheet directs when a GUID prediction fails: `R-MD1.xml` at s7r and s12r, `R-MDX.xml` at s12r, `R-MC2.xml` at s11r.

**Beyond the sheet, recorded and not judged:**
- The first try at step 3's name lost its spaces in the field (the desktop-control tool's key-by-key typing); it was seen by zoom before any Enter, erased by Backspace in that field, and retyped with a separate space key. Nothing was saved before the name read `BRIDGE Credit Note Manual`.
- The Debit Note type's creation screen showed one field step 3's did not, "Track Additional Costs for Purchases" (No), and no "Default jurisdiction" line. Neither was touched.
- The DESC/CMPINFO count lines moved after each import (LAB-COMMON C4: these are not rows). The V-CN answers at s6r and s7r differ only in five such lines, and at s8r and s9r only in three. Every answer's counts are in `evidence/cmpinfo.txt`.
- The Gateway's company line carried "(e)" and "(Schedule Backup)" at every reading.

## Step 1: load, check and back up
- Sheet files: all 33 equal to `SHA256.txt` at cec5d5a8. `.gitattributes`, `LAB-COMMON.md`, `run.py` and `M-marks.xml` are byte-equal to row 2346-AR's.
- Before the start, the book's data folder was found as the one whose `Company.1800` holds BRIDGE PILOT LAB and the book's GUID (one folder); it does not hold the excluded GUID (`evidence/bookcheck.txt`). The backup folder was made new and empty.
- Tally was started once, at 03:46:45Z, from its desktop shortcut, after Get-Process showed none. Its start-up company was shut first (deviation 2).
- C2 at 03:47:48Z, with Select Company showing: one `tally.exe`, which alone owns 9001; `GET /status` 200 (51 B); no company loaded.
- The book was loaded alone: `BRIDGE` and ` PILOT` typed into Select Company's name filter, one row, BRIDGE PILOT LAB, 1-Apr-26 to 31-Mar-27 (`s0-select`), Enter, no prompt. The Gateway (`s0-gateway`): current period 1-Apr-26 to 31-Mar-27; current date Tuesday, 15-Sep-2026 (not before 1-Aug-2026); date of last entry 15-Sep-26.
- `s1` (03:48:32Z): one COMPANY row, the book's GUID, ALTVCHID 68, ALTMSTID 245. EXPECT met.
- BK by C5 (03:50:24Z) to `C:\lab\2346-mn\backup-1` (Tally showed `C:\lab\2346-Mn\backup-1`): Data label, Backup & Restore, Backup; Backspace to the path; Specify Path; the path typed in three chunks; Enter, Enter; `BRIDGE` and ` PILOT` typed, the one row, Enter; End of List, Enter; "Accept ?" (`bk-accept`) = y. One file, 90,051 B; the Restore list showed it as 10-Nov-26 (9:20 am).

## Step 2: series, before anything is keyed
- `s2` (03:50:41Z): as in the Result. The three keyed names 0 rows each; the three predicted GUIDs held by none; V-CN, V-DN, R-MC1 and R-MD1 0 rows each. EXPECT met.

## Steps 3 to 5: the three types keyed
- Each from the Gateway: Create, Enter; List of Masters, Down 3 to Voucher Type, Enter; Voucher Type Creation. Name typed with a separate space key; (alias) Enter; Select type of voucher from the List of Voucher Types (Down 2 to Credit Note, or Down 3 to Debit Note); Abbreviation as Tally filled it (`C/Note`, `D/Note`); Activate (Yes); Method of Voucher Numbering from "Methods of Numbering" (Automatic, Automatic (Manual Override), Manual, Multi-user Auto, None), Down 2 to Manual. Once Manual was chosen Tally offered "Prevent creating duplicate Voucher Nos." showing No: y, Enter: Yes. No other field touched. Every highlight was read by zoom before its Enter.
- Every field and value as text: `evidence/s3-screens.txt`, `evidence/s4-screens.txt`, `evidence/s5-screens.txt`; the screen before each save: `s3-vt-before-save`, `s4-vt-before-save`, `s5-vt-before-save`.
- Each save: Ctrl+A, no prompt; a new blank Voucher Type Creation screen (`s<n>-new-blank-screen`); Ctrl+Q to Master Creation's List of Masters (`s<n>-master-creation-list`); Ctrl+Q to the Gateway, no dialog (`s<n>-gateway`).
- Before each, `s<n>m` and the before-write check passed. After each, `s<n>r` (T and M) and `count21.py --type-diff` against the earlier T: the earlier types 0 leaves differ (`evidence/typediff-s3r.txt`, `-s4r`, `-s5r`).
- At step 5, the desktop-control tool once refused a key because its own window held the keyboard focus; no key reached Tally. A click on Tally's title bar and a screenshot showed the screen unchanged before the next key.

## Steps 6 to 12: the notes and the reads
- Before each import, `s<n>m` and the before-write check passed (`evidence/beforewrite-s<n>m.txt`); the import was sent alone, and the window captured at once (`s<n>-after`): the Gateway, no dialog, each time.
- `s6r` `R-MC1.xml V-CN.xml M-marks.xml`; `s7r` `V-DN.xml V-CN.xml M-marks.xml`; `s8r` and `s9r` `R-MC1.xml V-CN.xml M-marks.xml`; `s10` the seven N reads; `s11r` `R-MC1.xml V-CN.xml N-CNc.xml M-marks.xml`; `s12r` `V-CN.xml N-CNc.xml N-CNd.xml M-marks.xml`. The R files left out are those of deviation 3.
- Step 8: `I-MC1-again.xml` checked byte-equal to `I-MC1.xml` before its send. `count21.py --diff a-s6r-R-MC1.xml a-s8r-R-MC1.xml` in `evidence/compare-s8r.txt`.
- Step 9: `count21.py --same-row a-s8r-M-marks.xml a-s9r-M-marks.xml`: equal (`evidence/samerow-s9r.txt`).
- The marks: ALTVCHID 69 after s6, 70 after s7, 71 after s8, 71 after s9, 72 after s11, 73 after s12; ALTMSTID 251 throughout.

## Step 13: restore (C6) and compare
- `prep-c6.sh 13` first: BK re-hashed equal to `evidence/bk.txt`, its folder holding only that file; `restore-plan.txt` written; the data folder listed (kept on the lab PC).
- K: Company label, Shut, Enter on BRIDGE PILOT LAB, no prompt; Data label, Backup & Restore, Restore; Specify Path; the path typed in three chunks; the list (`restore-list`) showed one row, BRIDGE PILOT LAB, 10-Nov-26 (9:20 am), as the plan; Enter on it; End of List, Enter; "Accept ?" (`restore-accept`) = y; "Company data folder (<company number>) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder." (`restore-replace`) = r. No other message.
- The book loaded alone (`s13-gateway`): period, date and last entry as at step 1.
- `s13` (04:07:44Z): as in the Result (`evidence/samerow-s13.txt`, `evidence/compare-s13.txt`).

## Step 14: close (C10)
- Shut by the K: Company label, Shut, BRIDGE PILOT LAB alone, Enter, no prompt: Select Company, no company loaded (`c10-no-company`, 04:08:21Z).
- C2 at 04:08:22Z, after the last request: one `tally.exe` (the same process as at step 0), which alone owns 9001; `GET /status` 200. No XML request after the shut.
- Tally was then closed by its own Quit (Q: Quit clicked at Select Company), no prompt; Get-Process then showed no `tally.exe`. The three Bridge processes were left running (deviation 1).

## Files
- `answers/`: every answer that may go, byte for byte (`kept-local.txt` lists those that stay on the lab PC, by size and sha256, with the reason).
- `evidence/`: notes, every `count21.py` output and compare, the screens' fields as text, the answers' sizes and sha256. Process and port listings, the backup-folder listing, the BK record and the data-folder listings stay on the lab PC.
- `logs/`: `run.py`'s log of each send.
- `scripts/`: the row's helpers, with lab paths replaced.
- `SCREENSHOTS.txt`: every capture, by name, size and sha256. They stay on the lab PC.

## count21.py outputs, whole

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
== a-s2-R-MC1.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s2-R-MD1.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s2-T-types.xml: 306946 bytes
  STATUS '1'
  VOUCHERTYPE rows: 28
  GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000e8: []
  GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000e9: []
  GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000ea: []
  named 'Credit Note': 1
   PARENT 'Credit Note'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000028'; NUMBERINGMETHOD 'Default'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Automatic'; NUMBERINGSUBMETHOD 'Auto Retain'; PREVENTDUPLICATES 'No'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [4]
   restart [1] DATE '20260401'
   restart [1] RESTARTFROM 'Yearly'
   restart [1] PERIODBEGINNIGNUM '1'
   unit leaf /ALLOWMULTIPLETAXUNITSPERSERIES[1] 'No'
  named 'Debit Note': 1
   PARENT 'Debit Note'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002a'; NUMBERINGMETHOD 'Default'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Automatic'; NUMBERINGSUBMETHOD 'Auto Retain'; PREVENTDUPLICATES 'No'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [4]
   restart [1] DATE '20260401'
   restart [1] RESTARTFROM 'Yearly'
   restart [1] PERIODBEGINNIGNUM '1'
   unit leaf /ALLOWMULTIPLETAXUNITSPERSERIES[1] 'No'
  named 'BRIDGE Credit Note Manual': 0
  named 'BRIDGE Debit Note Manual': 0
  named 'BRIDGE Credit Note Manual 2': 0
== a-s2-V-CN.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s2-V-DN.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
```

### s3m
```
== a-s3m-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s3r
```
== a-s3r-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '247'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s3r-T-types.xml: 319942 bytes
  STATUS '1'
  VOUCHERTYPE rows: 29
  GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000e8: ['BRIDGE Credit Note Manual']
  GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000e9: []
  GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000ea: []
  named 'Credit Note': 1
   PARENT 'Credit Note'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000028'; NUMBERINGMETHOD 'Default'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Automatic'; NUMBERINGSUBMETHOD 'Auto Retain'; PREVENTDUPLICATES 'No'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [4]
   restart [1] DATE '20260401'
   restart [1] RESTARTFROM 'Yearly'
   restart [1] PERIODBEGINNIGNUM '1'
   unit leaf /ALLOWMULTIPLETAXUNITSPERSERIES[1] 'No'
  named 'Debit Note': 1
   PARENT 'Debit Note'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002a'; NUMBERINGMETHOD 'Default'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Automatic'; NUMBERINGSUBMETHOD 'Auto Retain'; PREVENTDUPLICATES 'No'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [4]
   restart [1] DATE '20260401'
   restart [1] RESTARTFROM 'Yearly'
   restart [1] PERIODBEGINNIGNUM '1'
   unit leaf /ALLOWMULTIPLETAXUNITSPERSERIES[1] 'No'
  named 'BRIDGE Credit Note Manual': 1
   PARENT 'Credit Note'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000e8'; NUMBERINGMETHOD 'None'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Manual'; NUMBERINGSUBMETHOD 'Default'; PREVENTDUPLICATES 'Yes'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [0]
   unit leaf /TAXUNITNAME[1] 'Any'
   unit leaf /ALLOWMULTIPLETAXUNITSPERSERIES[1] 'No'
  named 'BRIDGE Debit Note Manual': 0
  named 'BRIDGE Credit Note Manual 2': 0
```

### s4m
```
== a-s4m-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '247'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s4r
```
== a-s4r-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '249'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s4r-T-types.xml: 332932 bytes
  STATUS '1'
  VOUCHERTYPE rows: 30
  GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000e8: ['BRIDGE Credit Note Manual']
  GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000e9: []
  GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000ea: ['BRIDGE Debit Note Manual']
  named 'Credit Note': 1
   PARENT 'Credit Note'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000028'; NUMBERINGMETHOD 'Default'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Automatic'; NUMBERINGSUBMETHOD 'Auto Retain'; PREVENTDUPLICATES 'No'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [4]
   restart [1] DATE '20260401'
   restart [1] RESTARTFROM 'Yearly'
   restart [1] PERIODBEGINNIGNUM '1'
   unit leaf /ALLOWMULTIPLETAXUNITSPERSERIES[1] 'No'
  named 'Debit Note': 1
   PARENT 'Debit Note'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002a'; NUMBERINGMETHOD 'Default'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Automatic'; NUMBERINGSUBMETHOD 'Auto Retain'; PREVENTDUPLICATES 'No'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [4]
   restart [1] DATE '20260401'
   restart [1] RESTARTFROM 'Yearly'
   restart [1] PERIODBEGINNIGNUM '1'
   unit leaf /ALLOWMULTIPLETAXUNITSPERSERIES[1] 'No'
  named 'BRIDGE Credit Note Manual': 1
   PARENT 'Credit Note'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000e8'; NUMBERINGMETHOD 'None'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Manual'; NUMBERINGSUBMETHOD 'Default'; PREVENTDUPLICATES 'Yes'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [0]
   unit leaf /TAXUNITNAME[1] 'Any'
   unit leaf /ALLOWMULTIPLETAXUNITSPERSERIES[1] 'No'
  named 'BRIDGE Debit Note Manual': 1
   PARENT 'Debit Note'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000ea'; NUMBERINGMETHOD 'None'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Manual'; NUMBERINGSUBMETHOD 'Default'; PREVENTDUPLICATES 'Yes'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [0]
   unit leaf /TAXUNITNAME[1] 'Any'
   unit leaf /ALLOWMULTIPLETAXUNITSPERSERIES[1] 'No'
  named 'BRIDGE Credit Note Manual 2': 0
```

### s5m
```
== a-s5m-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '249'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s5r
```
== a-s5r-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '251'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s5r-T-types.xml: 345940 bytes
  STATUS '1'
  VOUCHERTYPE rows: 31
  GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000e8: ['BRIDGE Credit Note Manual']
  GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000e9: []
  GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000ea: ['BRIDGE Debit Note Manual']
  named 'Credit Note': 1
   PARENT 'Credit Note'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000028'; NUMBERINGMETHOD 'Default'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Automatic'; NUMBERINGSUBMETHOD 'Auto Retain'; PREVENTDUPLICATES 'No'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [4]
   restart [1] DATE '20260401'
   restart [1] RESTARTFROM 'Yearly'
   restart [1] PERIODBEGINNIGNUM '1'
   unit leaf /ALLOWMULTIPLETAXUNITSPERSERIES[1] 'No'
  named 'Debit Note': 1
   PARENT 'Debit Note'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002a'; NUMBERINGMETHOD 'Default'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Automatic'; NUMBERINGSUBMETHOD 'Auto Retain'; PREVENTDUPLICATES 'No'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [4]
   restart [1] DATE '20260401'
   restart [1] RESTARTFROM 'Yearly'
   restart [1] PERIODBEGINNIGNUM '1'
   unit leaf /ALLOWMULTIPLETAXUNITSPERSERIES[1] 'No'
  named 'BRIDGE Credit Note Manual': 1
   PARENT 'Credit Note'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000e8'; NUMBERINGMETHOD 'None'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Manual'; NUMBERINGSUBMETHOD 'Default'; PREVENTDUPLICATES 'Yes'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [0]
   unit leaf /TAXUNITNAME[1] 'Any'
   unit leaf /ALLOWMULTIPLETAXUNITSPERSERIES[1] 'No'
  named 'BRIDGE Debit Note Manual': 1
   PARENT 'Debit Note'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000ea'; NUMBERINGMETHOD 'None'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Manual'; NUMBERINGSUBMETHOD 'Default'; PREVENTDUPLICATES 'Yes'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [0]
   unit leaf /TAXUNITNAME[1] 'Any'
   unit leaf /ALLOWMULTIPLETAXUNITSPERSERIES[1] 'No'
  named 'BRIDGE Credit Note Manual 2': 1
   PARENT 'Credit Note'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000ec'; NUMBERINGMETHOD 'None'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Manual'; NUMBERINGSUBMETHOD 'Default'; PREVENTDUPLICATES 'Yes'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [0]
   unit leaf /TAXUNITNAME[1] 'Any'
   unit leaf /ALLOWMULTIPLETAXUNITSPERSERIES[1] 'No'
```

### s6m
```
== a-s6m-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '251'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s6
```
== a-s6-I-MC1.xml: 540 bytes
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

### s6r
```
== a-s6r-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '251'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s6r-R-MC1.xml: 94548 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'CN/26-27/0001'
   VOUCHERTYPENAME 'BRIDGE Credit Note Manual'
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
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   ALTERID '69'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walk-in': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s6r-V-CN.xml: 52886 bytes
  STATUS '1'
  VOUCHER rows: 1
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID '45'
   ALTERID '69'
   DATE '20260801'
   VOUCHERNUMBER 'CN/26-27/0001'
   VOUCHERTYPENAME 'BRIDGE Credit Note Manual'
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
   REFERENCE 'BP/26-27/0010'
   REFERENCEDATE '20260801'
   GSTNATUREOFRETURN ''
   VATPARTYTRANSRETURNNUMBER ''
   VATPARTYTRANSRETURNDATE ''
   ISINVOICE 'Yes'
   VCHENTRYMODE 'Accounting Invoice'
   PERSISTEDVIEW 'Invoice Voucher View'
   GSTREGISTRATION attributes {}
   LEDGERENTRIES.LIST elements: 0
   ALLLEDGERENTRIES.LIST elements: 0
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   direct children: 729
```

### s7m
```
== a-s7m-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '69' ALTMSTID '251'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s7
```
== a-s7-I-MD1.xml: 540 bytes
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
```

### s7r
```
== a-s7r-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '251'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s7r-V-CN.xml: 52886 bytes
  STATUS '1'
  VOUCHER rows: 1
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID '45'
   ALTERID '69'
   DATE '20260801'
   VOUCHERNUMBER 'CN/26-27/0001'
   VOUCHERTYPENAME 'BRIDGE Credit Note Manual'
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
   REFERENCE 'BP/26-27/0010'
   REFERENCEDATE '20260801'
   GSTNATUREOFRETURN ''
   VATPARTYTRANSRETURNNUMBER ''
   VATPARTYTRANSRETURNDATE ''
   ISINVOICE 'Yes'
   VCHENTRYMODE 'Accounting Invoice'
   PERSISTEDVIEW 'Invoice Voucher View'
   GSTREGISTRATION attributes {}
   LEDGERENTRIES.LIST elements: 0
   ALLLEDGERENTRIES.LIST elements: 0
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   direct children: 729
== a-s7r-V-DN.xml: 52880 bytes
  STATUS '1'
  VOUCHER rows: 1
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002e'
   MASTERID '46'
   ALTERID '70'
   DATE '20260801'
   VOUCHERNUMBER 'DN/26-27/0001'
   VOUCHERTYPENAME 'BRIDGE Debit Note Manual'
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
   REFERENCE 'BP/26-27/0010'
   REFERENCEDATE '20260801'
   GSTNATUREOFRETURN ''
   VATPARTYTRANSRETURNNUMBER ''
   VATPARTYTRANSRETURNDATE ''
   ISINVOICE 'Yes'
   VCHENTRYMODE 'Accounting Invoice'
   PERSISTEDVIEW 'Invoice Voucher View'
   GSTREGISTRATION attributes {}
   LEDGERENTRIES.LIST elements: 0
   ALLLEDGERENTRIES.LIST elements: 0
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   direct children: 729
```

### s8m
```
== a-s8m-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '70' ALTMSTID '251'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s8
```
== a-s8-I-MC1-again.xml: 540 bytes
  root RESPONSE
  CREATED '0'
  ALTERED '1'
  DELETED '0'
  LASTVCHID '45'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '0'
```

### s8r
```
== a-s8r-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '71' ALTMSTID '251'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s8r-R-MC1.xml: 94548 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'CN/26-27/0001'
   VOUCHERTYPENAME 'BRIDGE Credit Note Manual'
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
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   ALTERID '71'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walk-in': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s8r-V-CN.xml: 52886 bytes
  STATUS '1'
  VOUCHER rows: 1
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID '45'
   ALTERID '71'
   DATE '20260801'
   VOUCHERNUMBER 'CN/26-27/0001'
   VOUCHERTYPENAME 'BRIDGE Credit Note Manual'
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
   REFERENCE 'BP/26-27/0010'
   REFERENCEDATE '20260801'
   GSTNATUREOFRETURN ''
   VATPARTYTRANSRETURNNUMBER ''
   VATPARTYTRANSRETURNDATE ''
   ISINVOICE 'Yes'
   VCHENTRYMODE 'Accounting Invoice'
   PERSISTEDVIEW 'Invoice Voucher View'
   GSTREGISTRATION attributes {}
   LEDGERENTRIES.LIST elements: 0
   ALLLEDGERENTRIES.LIST elements: 0
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   direct children: 729
```

### s9m
```
== a-s9m-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '71' ALTMSTID '251'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s9
```
== a-s9-I-MC1n.xml: 538 bytes
  root RESPONSE
  CREATED '0'
  ALTERED '0'
  DELETED '0'
  LASTVCHID '0'
  LASTMID '0'
  COMBINED '0'
  IGNORED '0'
  ERRORS '0'
  CANCELLED '0'
  EXCEPTIONS '1'
```

### s9r
```
== a-s9r-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '71' ALTMSTID '251'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s9r-R-MC1.xml: 94548 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'CN/26-27/0001'
   VOUCHERTYPENAME 'BRIDGE Credit Note Manual'
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
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   ALTERID '71'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walk-in': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s9r-V-CN.xml: 52886 bytes
  STATUS '1'
  VOUCHER rows: 1
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID '45'
   ALTERID '71'
   DATE '20260801'
   VOUCHERNUMBER 'CN/26-27/0001'
   VOUCHERTYPENAME 'BRIDGE Credit Note Manual'
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
   REFERENCE 'BP/26-27/0010'
   REFERENCEDATE '20260801'
   GSTNATUREOFRETURN ''
   VATPARTYTRANSRETURNNUMBER ''
   VATPARTYTRANSRETURNDATE ''
   ISINVOICE 'Yes'
   VCHENTRYMODE 'Accounting Invoice'
   PERSISTEDVIEW 'Invoice Voucher View'
   GSTREGISTRATION attributes {}
   LEDGERENTRIES.LIST elements: 0
   ALLLEDGERENTRIES.LIST elements: 0
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   direct children: 729
```

### s10
```
== a-s10-N-0010c.xml: 5624 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0010' DATE '20260801' VOUCHERTYPENAME 'BRIDGE Sales' BRIDGEVCHISCREDITNOTE 'No'
  rows with no class element: 0
== a-s10-N-0010d.xml: 5620 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0010' DATE '20260801' VOUCHERTYPENAME 'BRIDGE Sales' BRIDGEVCHISDEBITNOTE 'No'
  rows with no class element: 0
== a-s10-N-0010s.xml: 5606 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'BP/26-27/0010' DATE '20260801' VOUCHERTYPENAME 'BRIDGE Sales' BRIDGEVCHISSALES 'Yes'
  rows with no class element: 0
== a-s10-N-CNc.xml: 5676 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'CN/26-27/0001' DATE '20260801' VOUCHERTYPENAME 'BRIDGE Credit Note Manual' BRIDGEVCHISCREDITNOTE 'Yes'
  rows with no class element: 0
== a-s10-N-CNd.xml: 5670 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'CN/26-27/0001' DATE '20260801' VOUCHERTYPENAME 'BRIDGE Credit Note Manual' BRIDGEVCHISDEBITNOTE 'No'
  rows with no class element: 0
== a-s10-N-DNc.xml: 5670 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'DN/26-27/0001' DATE '20260801' VOUCHERTYPENAME 'BRIDGE Debit Note Manual' BRIDGEVCHISCREDITNOTE 'No'
  rows with no class element: 0
== a-s10-N-DNd.xml: 5668 bytes
  STATUS '1'
  VOUCHER rows: 1 of which with a VOUCHERNUMBER: 1 without one: 0
   'DN/26-27/0001' DATE '20260801' VOUCHERTYPENAME 'BRIDGE Debit Note Manual' BRIDGEVCHISDEBITNOTE 'Yes'
  rows with no class element: 0
```

### s11m
```
== a-s11m-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '71' ALTMSTID '251'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s11
```
== a-s11-I-MC2.xml: 540 bytes
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
```

### s11r
```
== a-s11r-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '72' ALTMSTID '251'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s11r-N-CNc.xml: 8332 bytes
  STATUS '1'
  VOUCHER rows: 2 of which with a VOUCHERNUMBER: 2 without one: 0
   'CN/26-27/0001' DATE '20260801' VOUCHERTYPENAME 'BRIDGE Credit Note Manual' BRIDGEVCHISCREDITNOTE 'Yes'
   'CN/26-27/0001' DATE '20260801' VOUCHERTYPENAME 'BRIDGE Credit Note Manual 2' BRIDGEVCHISCREDITNOTE 'Yes'
  rows with no class element: 0
== a-s11r-R-MC1.xml: 94548 bytes
  STATUS '1'
  VOUCHER rows: 1
   DATE '20260801'
   VOUCHERNUMBER 'CN/26-27/0001'
   VOUCHERTYPENAME 'BRIDGE Credit Note Manual'
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
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   ALTERID '71'
   VCHGSTSTATUSISINCLUDED 'Yes'
   VCHGSTSTATUSISUNCERTAIN 'No'
   VCHGSTSTATUSISEXCLUDED 'No'
   ISGSTOVERRIDDEN 'No'
   ISPOSTDATED 'No'
   GSTREGISTRATION <absent>
   CMPGSTIN <absent>
   leg 'BRIDGE Walk-in' | 'No' | '1051.00'
   leg 'BRIDGE Svc 998313 5%' | 'Yes' | '-600.40'
   leg 'BRIDGE Svc 998314 5%' | 'Yes' | '-400.40'
   leg 'BRIDGE CGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE SGST 2.5%' | 'Yes' | '-25.02'
   leg 'BRIDGE Round Off' | 'Yes' | '-0.16'
   bills on 'BRIDGE Walk-in': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998313 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Svc 998314 5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE CGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE SGST 2.5%': 0 with NAME, 1 whitespace-only, 1 lists
   bills on 'BRIDGE Round Off': 0 with NAME, 1 whitespace-only, 1 lists
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
== a-s11r-V-CN.xml: 102760 bytes
  STATUS '1'
  VOUCHER rows: 2
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID '45'
   ALTERID '71'
   DATE '20260801'
   VOUCHERNUMBER 'CN/26-27/0001'
   VOUCHERTYPENAME 'BRIDGE Credit Note Manual'
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
   REFERENCE 'BP/26-27/0010'
   REFERENCEDATE '20260801'
   GSTNATUREOFRETURN ''
   VATPARTYTRANSRETURNNUMBER ''
   VATPARTYTRANSRETURNDATE ''
   ISINVOICE 'Yes'
   VCHENTRYMODE 'Accounting Invoice'
   PERSISTEDVIEW 'Invoice Voucher View'
   GSTREGISTRATION attributes {}
   LEDGERENTRIES.LIST elements: 0
   ALLLEDGERENTRIES.LIST elements: 0
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   direct children: 729
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f'
   MASTERID '47'
   ALTERID '72'
   DATE '20260801'
   VOUCHERNUMBER 'CN/26-27/0001'
   VOUCHERTYPENAME 'BRIDGE Credit Note Manual 2'
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
   REFERENCE 'BP/26-27/0010'
   REFERENCEDATE '20260801'
   GSTNATUREOFRETURN ''
   VATPARTYTRANSRETURNNUMBER ''
   VATPARTYTRANSRETURNDATE ''
   ISINVOICE 'Yes'
   VCHENTRYMODE 'Accounting Invoice'
   PERSISTEDVIEW 'Invoice Voucher View'
   GSTREGISTRATION attributes {}
   LEDGERENTRIES.LIST elements: 0
   ALLLEDGERENTRIES.LIST elements: 0
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   direct children: 729
```

### s12m
```
== a-s12m-M-marks.xml: 3396 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '72' ALTMSTID '251'
  GUID is the book's: yes
  COMPANY rows: 1
```

### s12
```
== a-s12-I-MDX.xml: 540 bytes
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
```

### s12r
```
== a-s12r-M-marks.xml: 3398 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '73' ALTMSTID '251'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s12r-N-CNc.xml: 10976 bytes
  STATUS '1'
  VOUCHER rows: 3 of which with a VOUCHERNUMBER: 3 without one: 0
   'CN/26-27/0001' DATE '20260801' VOUCHERTYPENAME 'BRIDGE Credit Note Manual' BRIDGEVCHISCREDITNOTE 'Yes'
   'CN/26-27/0001' DATE '20260801' VOUCHERTYPENAME 'BRIDGE Credit Note Manual 2' BRIDGEVCHISCREDITNOTE 'Yes'
   'CN/26-27/0001' DATE '20260801' VOUCHERTYPENAME 'BRIDGE Debit Note Manual' BRIDGEVCHISCREDITNOTE 'No'
  rows with no class element: 0
== a-s12r-N-CNd.xml: 10962 bytes
  STATUS '1'
  VOUCHER rows: 3 of which with a VOUCHERNUMBER: 3 without one: 0
   'CN/26-27/0001' DATE '20260801' VOUCHERTYPENAME 'BRIDGE Credit Note Manual' BRIDGEVCHISDEBITNOTE 'No'
   'CN/26-27/0001' DATE '20260801' VOUCHERTYPENAME 'BRIDGE Credit Note Manual 2' BRIDGEVCHISDEBITNOTE 'No'
   'CN/26-27/0001' DATE '20260801' VOUCHERTYPENAME 'BRIDGE Debit Note Manual' BRIDGEVCHISDEBITNOTE 'Yes'
  rows with no class element: 0
== a-s12r-V-CN.xml: 152614 bytes
  STATUS '1'
  VOUCHER rows: 3
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002d'
   MASTERID '45'
   ALTERID '71'
   DATE '20260801'
   VOUCHERNUMBER 'CN/26-27/0001'
   VOUCHERTYPENAME 'BRIDGE Credit Note Manual'
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
   REFERENCE 'BP/26-27/0010'
   REFERENCEDATE '20260801'
   GSTNATUREOFRETURN ''
   VATPARTYTRANSRETURNNUMBER ''
   VATPARTYTRANSRETURNDATE ''
   ISINVOICE 'Yes'
   VCHENTRYMODE 'Accounting Invoice'
   PERSISTEDVIEW 'Invoice Voucher View'
   GSTREGISTRATION attributes {}
   LEDGERENTRIES.LIST elements: 0
   ALLLEDGERENTRIES.LIST elements: 0
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   direct children: 729
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002f'
   MASTERID '47'
   ALTERID '72'
   DATE '20260801'
   VOUCHERNUMBER 'CN/26-27/0001'
   VOUCHERTYPENAME 'BRIDGE Credit Note Manual 2'
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
   REFERENCE 'BP/26-27/0010'
   REFERENCEDATE '20260801'
   GSTNATUREOFRETURN ''
   VATPARTYTRANSRETURNNUMBER ''
   VATPARTYTRANSRETURNDATE ''
   ISINVOICE 'Yes'
   VCHENTRYMODE 'Accounting Invoice'
   PERSISTEDVIEW 'Invoice Voucher View'
   GSTREGISTRATION attributes {}
   LEDGERENTRIES.LIST elements: 0
   ALLLEDGERENTRIES.LIST elements: 0
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   direct children: 729
   GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000030'
   MASTERID '48'
   ALTERID '73'
   DATE '20260801'
   VOUCHERNUMBER 'CN/26-27/0001'
   VOUCHERTYPENAME 'BRIDGE Debit Note Manual'
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
   REFERENCE 'BP/26-27/0010'
   REFERENCEDATE '20260801'
   GSTNATUREOFRETURN ''
   VATPARTYTRANSRETURNNUMBER ''
   VATPARTYTRANSRETURNDATE ''
   ISINVOICE 'Yes'
   VCHENTRYMODE 'Accounting Invoice'
   PERSISTEDVIEW 'Invoice Voucher View'
   GSTREGISTRATION attributes {}
   LEDGERENTRIES.LIST elements: 0
   ALLLEDGERENTRIES.LIST elements: 0
   ALLINVENTORYENTRIES.LIST elements: 0
   INVENTORYENTRIES.LIST elements: 0
   LEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   ALLLEDGERENTRIES.LIST INVENTORYALLOCATIONS.LIST elements: 0
   direct children: 729
```

### s13
```
== a-s13-M-marks.xml: 3394 bytes
  STATUS '1'
  COMPANY '6b43e498-430c-4d5c-bfef-d32e2ab93c85' ALTVCHID '68' ALTMSTID '245'
  GUID is the book's: yes
  COMPANY rows: 1
== a-s13-T-types.xml: 306946 bytes
  STATUS '1'
  VOUCHERTYPE rows: 28
  GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000e8: []
  GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000e9: []
  GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85-000000ea: []
  named 'Credit Note': 1
   PARENT 'Credit Note'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-00000028'; NUMBERINGMETHOD 'Default'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Automatic'; NUMBERINGSUBMETHOD 'Auto Retain'; PREVENTDUPLICATES 'No'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [4]
   restart [1] DATE '20260401'
   restart [1] RESTARTFROM 'Yearly'
   restart [1] PERIODBEGINNIGNUM '1'
   unit leaf /ALLOWMULTIPLETAXUNITSPERSERIES[1] 'No'
  named 'Debit Note': 1
   PARENT 'Debit Note'; GUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85-0000002a'; NUMBERINGMETHOD 'Default'; ISACTIVE 'Yes'
   VOUCHERNUMBERSERIES.LIST elements: 1
   series: NAME 'Default'; NUMBERINGMETHOD 'Automatic'; NUMBERINGSUBMETHOD 'Auto Retain'; PREVENTDUPLICATES 'No'; DUPLICATECONTROL ''; BEGINNINGNUMBER '1'
   RESTARTFROMLIST.LIST elements: 1 children: [4]
   restart [1] DATE '20260401'
   restart [1] RESTARTFROM 'Yearly'
   restart [1] PERIODBEGINNIGNUM '1'
   unit leaf /ALLOWMULTIPLETAXUNITSPERSERIES[1] 'No'
  named 'BRIDGE Credit Note Manual': 0
  named 'BRIDGE Debit Note Manual': 0
  named 'BRIDGE Credit Note Manual 2': 0
== a-s13-V-CN.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
== a-s13-V-DN.xml: 3022 bytes
  STATUS '1'
  VOUCHER rows: 0
```

## Compares, checks and screens, whole

### step 3: the Voucher Type Creation screen before the save, as text
```
Step 3, Voucher Type Creation (title bar "Voucher Type Creation", BRIDGE PILOT LAB), every field and value as the screen showed it before the save (shots/s3-vt-before-save.png), cursor on "Use effective dates for vouchers":
Name: BRIDGE Credit Note Manual
(alias): (blank)
General
Select type of voucher: Credit Note
Abbreviation: C/Note (Tally's fill)
Activate this Voucher Type: Yes
Method of Voucher Numbering: Manual
  Prevent creating duplicate Voucher Nos.: Yes
Show unused vch nos. in transactions for Retain Original Voucher No. behaviour: No (greyed)
Use effective dates for vouchers: No
Allow zero-valued transactions: No
Make this voucher type as 'Optional' by default: No
Allow narration in voucher: Yes
Provide narrations for each ledger in voucher: No
Enable default accounting allocations: No
WhatsApp voucher after saving: No
Printing
Print voucher after saving: No
Default jurisdiction: (blank)
Default template to print (Invoice Mode): Not Applicable
Default title to print: (blank)
Set/Alter Terms and Conditions: No
Name of Class: (blank)
```

### step 4: the same
```
Step 4, Voucher Type Creation (title bar "Voucher Type Creation", BRIDGE PILOT LAB), every field and value as the screen showed it before the save (shots/s4-vt-before-save.png), cursor on "Use effective dates for vouchers":
Name: BRIDGE Debit Note Manual
(alias): (blank)
General
Select type of voucher: Debit Note
Abbreviation: D/Note (Tally's fill)
Activate this Voucher Type: Yes
Method of Voucher Numbering: Manual
  Prevent creating duplicate Voucher Nos.: Yes
Show unused vch nos. in transactions for Retain Original Voucher No. behaviour: No (greyed)
Use effective dates for vouchers: No
Allow zero-valued transactions: No
Make this voucher type as 'Optional' by default: No
Allow narration in voucher: Yes
Provide narrations for each ledger in voucher: No
Track Additional Costs for Purchases: No (shown on this type's screen, not on step 3's)
Enable default accounting allocations: No
WhatsApp voucher after saving: No
Printing
Print voucher after saving: No
Default template to print (Invoice Mode): Not Applicable
Default title to print: (blank)
Set/Alter Terms and Conditions: No
(no "Default jurisdiction" line on this type's screen, unlike step 3's)
Name of Class: (blank)
```

### step 5: the same
```
Step 5, Voucher Type Creation (title bar "Voucher Type Creation", BRIDGE PILOT LAB), every field and value as the screen showed it before the save (shots/s5-vt-before-save.png), cursor on "Use effective dates for vouchers":
Name: BRIDGE Credit Note Manual 2
(alias): (blank)
General
Select type of voucher: Credit Note
Abbreviation: C/Note (Tally's fill)
Activate this Voucher Type: Yes
Method of Voucher Numbering: Manual
  Prevent creating duplicate Voucher Nos.: Yes
Show unused vch nos. in transactions for Retain Original Voucher No. behaviour: No (greyed)
Use effective dates for vouchers: No
Allow zero-valued transactions: No
Make this voucher type as 'Optional' by default: No
Allow narration in voucher: Yes
Provide narrations for each ledger in voucher: No
Enable default accounting allocations: No
WhatsApp voucher after saving: No
Printing
Print voucher after saving: No
Default jurisdiction: (blank)
Default template to print (Invoice Mode): Not Applicable
Default title to print: (blank)
Set/Alter Terms and Conditions: No
Name of Class: (blank)
```

### s3m: M against s1 (the before-write check)
```
== a-s3m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s4m: M against s1 (the before-write check)
```
== a-s4m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'247'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s5m: M against s1 (the before-write check)
```
== a-s5m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'249'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s6m: M against s1 (the before-write check)
```
== a-s6m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'251'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s7m: M against s1 (the before-write check)
```
== a-s7m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'69'", "'251'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s8m: M against s1 (the before-write check)
```
== a-s8m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'70'", "'251'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s9m: M against s1 (the before-write check)
```
== a-s9m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'251'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s11m: M against s1 (the before-write check)
```
== a-s11m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'251'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s12m: M against s1 (the before-write check)
```
== a-s12m-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'72'", "'251'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  one COMPANY row, the same GUID: the write may be sent
```

### s3r: T against s2 (--type-diff)
```
== --type-diff a-s2-T-types.xml a-s3r-T-types.xml: VOUCHERTYPE rows 28 -> 29
   type 'BRIDGE Credit Note Manual': in the second only
   type 'Credit Note': 0 leaves differ
   type 'Debit Note': 0 leaves differ
```

### s4r: T against s3r (--type-diff)
```
== --type-diff a-s3r-T-types.xml a-s4r-T-types.xml: VOUCHERTYPE rows 29 -> 30
   type 'BRIDGE Credit Note Manual': 0 leaves differ
   type 'BRIDGE Debit Note Manual': in the second only
   type 'Credit Note': 0 leaves differ
   type 'Debit Note': 0 leaves differ
```

### s5r: T against s4r (--type-diff)
```
== --type-diff a-s4r-T-types.xml a-s5r-T-types.xml: VOUCHERTYPE rows 30 -> 31
   type 'BRIDGE Credit Note Manual': 0 leaves differ
   type 'BRIDGE Credit Note Manual 2': in the second only
   type 'BRIDGE Debit Note Manual': 0 leaves differ
   type 'Credit Note': 0 leaves differ
   type 'Debit Note': 0 leaves differ
```

### s6r and s7r: the V-CN lines that differ
```
a-s6r-V-CN.xml against a-s7r-V-CN.xml: lines a=788, b=788
  line 11: a <LEDGER>11</LEDGER> / b <LEDGER>22</LEDGER>
  line 19: a <CURRENCY>1</CURRENCY> / b <CURRENCY>2</CURRENCY>
  line 39: a <TAXUNIT>1</TAXUNIT> / b <TAXUNIT>2</TAXUNIT>
  line 42: a <VOUCHERNUMBERSERIES>1</VOUCHERNUMBERSERIES> / b <VOUCHERNUMBERSERIES>2</VOUCHERNUMBERSERIES>
  line 43: a <VOUCHER>2</VOUCHER> / b <VOUCHER>4</VOUCHER>
```

### s8r: R-MC1 against s6r (--diff)
```
== --diff a-s6r-R-MC1.xml a-s8r-R-MC1.xml
   ALTERID: '69' -> '71'
   lists compared: 1
   legs: 6 -> 6
```

### s8r and s9r: the V-CN lines that differ
```
a-s8r-V-CN.xml against a-s9r-V-CN.xml: lines a=788, b=788
  line 11: a <LEDGER>44</LEDGER> / b <LEDGER>55</LEDGER>
  line 39: a <TAXUNIT>4</TAXUNIT> / b <TAXUNIT>5</TAXUNIT>
  line 42: a <VOUCHERNUMBERSERIES>5</VOUCHERNUMBERSERIES> / b <VOUCHERNUMBERSERIES>6</VOUCHERNUMBERSERIES>
```

### s8r and s9r: the R-MC1 lines that differ
```
a-s8r-R-MC1.xml against a-s9r-R-MC1.xml: lines a=1311, b=1311
  line 11: a <LEDGER>44</LEDGER> / b <LEDGER>55</LEDGER>
  line 39: a <TAXUNIT>4</TAXUNIT> / b <TAXUNIT>5</TAXUNIT>
  line 42: a <VOUCHERNUMBERSERIES>5</VOUCHERNUMBERSERIES> / b <VOUCHERNUMBERSERIES>6</VOUCHERNUMBERSERIES>
```

### s9r: M against s8r (--same-row)
```
== a-s9r-M-marks.xml against a-s8r-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'251'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'71'", "'251'")
  equal
```

### s13: M against s1 (--same-row)
```
== a-s13-M-marks.xml against a-s1-M-marks.xml: GUID, ALTVCHID, ALTMSTID ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'") against ("'6b43e498-430c-4d5c-bfef-d32e2ab93c85'", "'68'", "'245'")
  equal
```

### s13: T, V-CN and V-DN against s2 by sha256
```
T-types s2 cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed s13 cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed EQUAL
V-CN s2 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 s13 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 EQUAL
V-DN s2 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 s13 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974 EQUAL
```

### DESC/CMPINFO count lines of every answer
```
$ the DESC/CMPINFO count lines of every read answer that has them (LAB-COMMON C4: not rows); the names with a count other than 0, then the rest as "others 0"
a-s1-M-marks.xml: all 0 (names in all: 35)
a-s2-T-types.xml: all 0 (names in all: 35)
a-s2-V-CN.xml: all 0 (names in all: 35)
a-s2-V-DN.xml: all 0 (names in all: 35)
a-s2-R-MC1.xml: all 0 (names in all: 35)
a-s2-R-MD1.xml: all 0 (names in all: 35)
a-s2-M-marks.xml: all 0 (names in all: 35)
a-s3m-M-marks.xml: all 0 (names in all: 35)
a-s3r-T-types.xml: VOUCHERTYPE 1; others 0 (names in all: 35)
a-s3r-M-marks.xml: VOUCHERTYPE 1; others 0 (names in all: 35)
a-s4m-M-marks.xml: VOUCHERTYPE 1; others 0 (names in all: 35)
a-s4r-T-types.xml: VOUCHERTYPE 2; others 0 (names in all: 35)
a-s4r-M-marks.xml: VOUCHERTYPE 2; others 0 (names in all: 35)
a-s5m-M-marks.xml: VOUCHERTYPE 2; others 0 (names in all: 35)
a-s5r-T-types.xml: VOUCHERTYPE 3; others 0 (names in all: 35)
a-s5r-M-marks.xml: VOUCHERTYPE 3; others 0 (names in all: 35)
a-s6m-M-marks.xml: VOUCHERTYPE 3; others 0 (names in all: 35)
a-s6r-R-MC1.xml: LEDGER 11; VOUCHERTYPE 3; CURRENCY 1; TAXUNIT 1; VOUCHERNUMBERSERIES 1; VOUCHER 2; others 0 (names in all: 35)
a-s6r-V-CN.xml: LEDGER 11; VOUCHERTYPE 3; CURRENCY 1; TAXUNIT 1; VOUCHERNUMBERSERIES 1; VOUCHER 2; others 0 (names in all: 35)
a-s6r-M-marks.xml: LEDGER 11; VOUCHERTYPE 3; CURRENCY 1; TAXUNIT 1; VOUCHERNUMBERSERIES 1; VOUCHER 2; others 0 (names in all: 35)
a-s7m-M-marks.xml: LEDGER 11; VOUCHERTYPE 3; CURRENCY 1; TAXUNIT 1; VOUCHERNUMBERSERIES 1; VOUCHER 2; others 0 (names in all: 35)
a-s7r-V-DN.xml: LEDGER 22; VOUCHERTYPE 3; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 4; others 0 (names in all: 35)
a-s7r-V-CN.xml: LEDGER 22; VOUCHERTYPE 3; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 4; others 0 (names in all: 35)
a-s7r-M-marks.xml: LEDGER 22; VOUCHERTYPE 3; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 4; others 0 (names in all: 35)
a-s8m-M-marks.xml: LEDGER 22; VOUCHERTYPE 3; CURRENCY 2; TAXUNIT 2; VOUCHERNUMBERSERIES 2; VOUCHER 4; others 0 (names in all: 35)
a-s8r-R-MC1.xml: LEDGER 44; VOUCHERTYPE 3; CURRENCY 3; TAXUNIT 4; VOUCHERNUMBERSERIES 5; VOUCHER 6; others 0 (names in all: 35)
a-s8r-V-CN.xml: LEDGER 44; VOUCHERTYPE 3; CURRENCY 3; TAXUNIT 4; VOUCHERNUMBERSERIES 5; VOUCHER 6; others 0 (names in all: 35)
a-s8r-M-marks.xml: LEDGER 44; VOUCHERTYPE 3; CURRENCY 3; TAXUNIT 4; VOUCHERNUMBERSERIES 5; VOUCHER 6; others 0 (names in all: 35)
a-s9m-M-marks.xml: LEDGER 44; VOUCHERTYPE 3; CURRENCY 3; TAXUNIT 4; VOUCHERNUMBERSERIES 5; VOUCHER 6; others 0 (names in all: 35)
a-s9r-R-MC1.xml: LEDGER 55; VOUCHERTYPE 3; CURRENCY 3; TAXUNIT 5; VOUCHERNUMBERSERIES 6; VOUCHER 6; others 0 (names in all: 35)
a-s9r-V-CN.xml: LEDGER 55; VOUCHERTYPE 3; CURRENCY 3; TAXUNIT 5; VOUCHERNUMBERSERIES 6; VOUCHER 6; others 0 (names in all: 35)
a-s9r-M-marks.xml: LEDGER 55; VOUCHERTYPE 3; CURRENCY 3; TAXUNIT 5; VOUCHERNUMBERSERIES 6; VOUCHER 6; others 0 (names in all: 35)
a-s10-N-0010s.xml: LEDGER 55; VOUCHERTYPE 3; CURRENCY 3; TAXUNIT 5; VOUCHERNUMBERSERIES 6; VOUCHER 6; others 0 (names in all: 35)
a-s10-N-0010c.xml: LEDGER 55; VOUCHERTYPE 3; CURRENCY 3; TAXUNIT 5; VOUCHERNUMBERSERIES 6; VOUCHER 6; others 0 (names in all: 35)
a-s10-N-0010d.xml: LEDGER 55; VOUCHERTYPE 3; CURRENCY 3; TAXUNIT 5; VOUCHERNUMBERSERIES 6; VOUCHER 6; others 0 (names in all: 35)
a-s10-N-CNc.xml: LEDGER 55; VOUCHERTYPE 3; CURRENCY 3; TAXUNIT 5; VOUCHERNUMBERSERIES 6; VOUCHER 6; others 0 (names in all: 35)
a-s10-N-CNd.xml: LEDGER 55; VOUCHERTYPE 3; CURRENCY 3; TAXUNIT 5; VOUCHERNUMBERSERIES 6; VOUCHER 6; others 0 (names in all: 35)
a-s10-N-DNc.xml: LEDGER 55; VOUCHERTYPE 3; CURRENCY 3; TAXUNIT 5; VOUCHERNUMBERSERIES 6; VOUCHER 6; others 0 (names in all: 35)
a-s10-N-DNd.xml: LEDGER 55; VOUCHERTYPE 3; CURRENCY 3; TAXUNIT 5; VOUCHERNUMBERSERIES 6; VOUCHER 6; others 0 (names in all: 35)
a-s11m-M-marks.xml: LEDGER 55; VOUCHERTYPE 3; CURRENCY 3; TAXUNIT 5; VOUCHERNUMBERSERIES 6; VOUCHER 6; others 0 (names in all: 35)
a-s11r-R-MC1.xml: LEDGER 66; VOUCHERTYPE 3; CURRENCY 4; TAXUNIT 6; VOUCHERNUMBERSERIES 7; VOUCHER 8; others 0 (names in all: 35)
a-s11r-V-CN.xml: LEDGER 66; VOUCHERTYPE 3; CURRENCY 4; TAXUNIT 6; VOUCHERNUMBERSERIES 7; VOUCHER 8; others 0 (names in all: 35)
a-s11r-N-CNc.xml: LEDGER 66; VOUCHERTYPE 3; CURRENCY 4; TAXUNIT 6; VOUCHERNUMBERSERIES 7; VOUCHER 8; others 0 (names in all: 35)
a-s11r-M-marks.xml: LEDGER 66; VOUCHERTYPE 3; CURRENCY 4; TAXUNIT 6; VOUCHERNUMBERSERIES 7; VOUCHER 8; others 0 (names in all: 35)
a-s12m-M-marks.xml: LEDGER 66; VOUCHERTYPE 3; CURRENCY 4; TAXUNIT 6; VOUCHERNUMBERSERIES 7; VOUCHER 8; others 0 (names in all: 35)
a-s12r-V-CN.xml: LEDGER 77; VOUCHERTYPE 3; CURRENCY 5; TAXUNIT 7; VOUCHERNUMBERSERIES 8; VOUCHER 10; others 0 (names in all: 35)
a-s12r-N-CNc.xml: LEDGER 77; VOUCHERTYPE 3; CURRENCY 5; TAXUNIT 7; VOUCHERNUMBERSERIES 8; VOUCHER 10; others 0 (names in all: 35)
a-s12r-N-CNd.xml: LEDGER 77; VOUCHERTYPE 3; CURRENCY 5; TAXUNIT 7; VOUCHERNUMBERSERIES 8; VOUCHER 10; others 0 (names in all: 35)
a-s12r-M-marks.xml: LEDGER 77; VOUCHERTYPE 3; CURRENCY 5; TAXUNIT 7; VOUCHERNUMBERSERIES 8; VOUCHER 10; others 0 (names in all: 35)
a-s13-M-marks.xml: all 0 (names in all: 35)
a-s13-T-types.xml: all 0 (names in all: 35)
a-s13-V-CN.xml: all 0 (names in all: 35)
a-s13-V-DN.xml: all 0 (names in all: 35)
```

### sha256 of every answer
```
$ sha256sum and size of every answer, in send order
a-s1-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s2-T-types.xml 306946 B cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed
a-s2-V-CN.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-V-DN.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-R-MC1.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-R-MD1.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s2-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s3m-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s3r-T-types.xml 319942 B a833383a3e1b48c48054a03de396d31b92e0d2524c05510afb031fb8f54f7e70
a-s3r-M-marks.xml 3394 B 913425f72bab50b0f29ac48c7c650ecacb0bebab076434ab524ec85521561196
a-s4m-M-marks.xml 3394 B 913425f72bab50b0f29ac48c7c650ecacb0bebab076434ab524ec85521561196
a-s4r-T-types.xml 332932 B 1034da1b052bf91b41213b06e7434e2663720a326864501cdb19181048b03d94
a-s4r-M-marks.xml 3394 B 4f940345dcacb4a304588e50c2ea2e64ccdf2f908e35a58aea8ecd3806256921
a-s5m-M-marks.xml 3394 B 4f940345dcacb4a304588e50c2ea2e64ccdf2f908e35a58aea8ecd3806256921
a-s5r-T-types.xml 345940 B 8b72cf04686f88e13d9996f9968e12df707560069589bfd6bf9e80927fec4c9e
a-s5r-M-marks.xml 3394 B db2cc1dc4922e4ebd542450e0d3546e38d1718b0c12ff26735ec4b52d752343c
a-s6m-M-marks.xml 3394 B db2cc1dc4922e4ebd542450e0d3546e38d1718b0c12ff26735ec4b52d752343c
a-s6-I-MC1.xml 540 B fdcd8435c6fcf95ab624cbe2cff2c887078df500626f7c9c37cfa834a1cdb6ea
a-s6r-R-MC1.xml 94548 B 978a0d031968681b21ad004dbf9fbe26fcd4ec230f22459096f834d528bab113
a-s6r-V-CN.xml 52886 B b50bb7cc3747125e1b0ed7bde9bcc6aad56d70da276466887fab3106f98ad24e
a-s6r-M-marks.xml 3396 B d4f3d87938b2e0c5caca034f770358be50304fe2c39167f3d0afdbd9b305fd45
a-s7m-M-marks.xml 3396 B d4f3d87938b2e0c5caca034f770358be50304fe2c39167f3d0afdbd9b305fd45
a-s7-I-MD1.xml 540 B 32e7a29b201c773547aa47bc95bd3eda8af83aeb6e35090303b9b27bcd34eec5
a-s7r-V-DN.xml 52880 B 3a595df1170eb3c30a4f41d188c74e94b2070e766eeac4ecb17ae5e1252163ee
a-s7r-V-CN.xml 52886 B e40da49cbe2a0c35054768638ca174d7b1501db7e51ff105defc1490ca30e824
a-s7r-M-marks.xml 3396 B cfffcc638695f621f790335d027113fa18f1c108e17e0bd2b8dcae6d75c43388
a-s8m-M-marks.xml 3396 B cfffcc638695f621f790335d027113fa18f1c108e17e0bd2b8dcae6d75c43388
a-s8-I-MC1-again.xml 540 B 00ecc8e140cbd17a8fa2b235a01886d5f58a7c8de3bb4c34032a676b9b8a1308
a-s8r-R-MC1.xml 94548 B 7b1a0020a4265de2fd60056b562221094d6732e0e429095ef75c403c1aaf10ec
a-s8r-V-CN.xml 52886 B 3995d7c9ffc71bfcfbc9000ec6ce86d6dab83331a65781bd18b3c96ecb50bcc1
a-s8r-M-marks.xml 3396 B 9085f83a43c090b50e158bc23309ba5435def2e9572483a143317c6e05f2330a
a-s9m-M-marks.xml 3396 B 9085f83a43c090b50e158bc23309ba5435def2e9572483a143317c6e05f2330a
a-s9-I-MC1n.xml 538 B 09a713e2674c7016c2cb10f194f6e0fc7f6bf9dee3b523f4f6bbf1b07ea0f874
a-s9r-R-MC1.xml 94548 B 249b04508be0881a1734fe46fa61ae0f9114acb9dba7f9088672e345a4cd2136
a-s9r-V-CN.xml 52886 B 77600c2fdcc1d8aca4f3a54df168a59e22a0063e3c58103e98810dc3447e846b
a-s9r-M-marks.xml 3396 B 0431ba7efa701aae5a6e1dbf4b8f5b8e51924ff308f6c3d0500fe57a2b01fed5
a-s10-N-0010s.xml 5606 B a976920d34a5f6dacaa07215d1f603d5c56f5b0dd2133b3633add665c40520ed
a-s10-N-0010c.xml 5624 B 2b5f923ce5ab307a4eb930dde266796c1e2aa3a700e6dae0973c971a127629e8
a-s10-N-0010d.xml 5620 B b7676e141e78ae79e135e55507c0bc88cb28400d0e386a0cf2dd7542ab72abca
a-s10-N-CNc.xml 5676 B 7dd487775b4cab0912a0c045b45dedc4a57da3c87eab190fcab472557cbe44ed
a-s10-N-CNd.xml 5670 B 5c43f8d199a8860a90f4d079e8fa9715373f275735cab81285011be31ca8786d
a-s10-N-DNc.xml 5670 B 59dd897a5625ffb354ecd3686ebc9aaaa760da48b4e0b63c802998c77b465f7e
a-s10-N-DNd.xml 5668 B ee64978d609167cd264391710d2e3cd41453ebae222d1bf198a62af56f267978
a-s11m-M-marks.xml 3396 B 0431ba7efa701aae5a6e1dbf4b8f5b8e51924ff308f6c3d0500fe57a2b01fed5
a-s11-I-MC2.xml 540 B f8384a874e1d8a6be92c2969f94e2887fb700768f6e111b6b3f649cd13ffdc7e
a-s11r-R-MC1.xml 94548 B e888365d4448137bc786c03b9e3c0872a72fbf8e39f0cfdfe9fd91c1508bfd96
a-s11r-V-CN.xml 102760 B 67e8612ff07a00d958dba8801b64de189b21cdb897ca4ac604a249d72a595d5e
a-s11r-N-CNc.xml 8332 B 6a485d5e8386e82d223698fdb9a5f413b297db666ebae38392bd6d4b7f581c1a
a-s11r-M-marks.xml 3396 B d01f87d84d557e56dc1bc07b9ba405a07d1527bd64912b064bfc7ca8809b6aa9
a-s12m-M-marks.xml 3396 B d01f87d84d557e56dc1bc07b9ba405a07d1527bd64912b064bfc7ca8809b6aa9
a-s12-I-MDX.xml 540 B e6540f6534095f5608f3c59f802c1f40a3ae13529a7e9041847b460f03c32fec
a-s12r-V-CN.xml 152614 B c61e81da5e24f8b627ec76bdd0c13948dd0bb70a2ba8856723c48d2e5df551b5
a-s12r-N-CNc.xml 10976 B 5396c3f0410d3a128ea90adc6f1fec4d0aa4719bfba458d1c478bfd7c39f25c5
a-s12r-N-CNd.xml 10962 B df0042221bc05986d97c6e3eb991623f238f2d4cc7093421c2f9db5c3f9af789
a-s12r-M-marks.xml 3398 B 61d60adeac7874695c4a9536070753bed8f07792e103def7bf6c71ffdf427e56
a-s13-M-marks.xml 3394 B 8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6
a-s13-T-types.xml 306946 B cfd362721f9dd1f39dd7033d90a839999341a85df6c3ac0ac109384d236197ed
a-s13-V-CN.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
a-s13-V-DN.xml 3022 B 947fbf369d0fcf74e53bff1a3802a927c72737735dbb3064b3cfe34dd70fa974
```

### each send's start, UTC
```
2026-11-10T03:48:32.959Z s1 M-marks.xml (hashes equal to the lists)
2026-11-10T03:50:41.691Z s2 T-types.xml V-CN.xml V-DN.xml R-MC1.xml R-MD1.xml M-marks.xml (hashes equal to the lists)
2026-11-10T03:51:04.989Z s3m M-marks.xml (hashes equal to the lists)
2026-11-10T03:54:23.486Z s3r T-types.xml M-marks.xml (hashes equal to the lists)
2026-11-10T03:54:46.798Z s4m M-marks.xml (hashes equal to the lists)
2026-11-10T03:56:55.500Z s4r T-types.xml M-marks.xml (hashes equal to the lists)
2026-11-10T03:57:22.955Z s5m M-marks.xml (hashes equal to the lists)
2026-11-10T03:59:52.356Z s5r T-types.xml M-marks.xml (hashes equal to the lists)
2026-11-10T04:00:10.310Z s6m M-marks.xml (hashes equal to the lists)
2026-11-10T04:00:18.188Z s6 I-MC1.xml (hashes equal to the lists)
2026-11-10T04:00:37.742Z s6r R-MC1.xml V-CN.xml M-marks.xml (hashes equal to the lists)
2026-11-10T04:00:58.516Z s7m M-marks.xml (hashes equal to the lists)
2026-11-10T04:01:07.183Z s7 I-MD1.xml (hashes equal to the lists)
2026-11-10T04:01:23.913Z s7r V-DN.xml V-CN.xml M-marks.xml (hashes equal to the lists)
2026-11-10T04:01:37.910Z s8m M-marks.xml (hashes equal to the lists)
2026-11-10T04:02:06.883Z s8 I-MC1-again.xml (hashes equal to the lists)
2026-11-10T04:02:20.173Z s8r R-MC1.xml V-CN.xml M-marks.xml (hashes equal to the lists)
2026-11-10T04:02:33.138Z s9m M-marks.xml (hashes equal to the lists)
2026-11-10T04:02:41.090Z s9 I-MC1n.xml (hashes equal to the lists)
2026-11-10T04:02:54.546Z s9r R-MC1.xml V-CN.xml M-marks.xml (hashes equal to the lists)
2026-11-10T04:03:17.404Z s10 N-0010s.xml N-0010c.xml N-0010d.xml N-CNc.xml N-CNd.xml N-DNc.xml N-DNd.xml (hashes equal to the lists)
2026-11-10T04:03:29.141Z s11m M-marks.xml (hashes equal to the lists)
2026-11-10T04:03:37.238Z s11 I-MC2.xml (hashes equal to the lists)
2026-11-10T04:03:52.082Z s11r R-MC1.xml V-CN.xml N-CNc.xml M-marks.xml (hashes equal to the lists)
2026-11-10T04:04:08.338Z s12m M-marks.xml (hashes equal to the lists)
2026-11-10T04:04:16.399Z s12 I-MDX.xml (hashes equal to the lists)
2026-11-10T04:04:30.772Z s12r V-CN.xml N-CNc.xml N-CNd.xml M-marks.xml (hashes equal to the lists)
2026-11-10T04:07:44.106Z s13 M-marks.xml T-types.xml V-CN.xml V-DN.xml (hashes equal to the lists)
```

## run.py logs (each send: local IST time to the second, request, answer file, curl exit, bytes)

### s1
```
09:18:33	M-marks.xml	a-s1-M-marks.xml	curl 0	3394 bytes
```

### s2
```
09:20:42	T-types.xml	a-s2-T-types.xml	curl 0	306946 bytes
09:20:42	V-CN.xml	a-s2-V-CN.xml	curl 0	3022 bytes
09:20:42	V-DN.xml	a-s2-V-DN.xml	curl 0	3022 bytes
09:20:42	R-MC1.xml	a-s2-R-MC1.xml	curl 0	3022 bytes
09:20:42	R-MD1.xml	a-s2-R-MD1.xml	curl 0	3022 bytes
09:20:42	M-marks.xml	a-s2-M-marks.xml	curl 0	3394 bytes
```

### s3m
```
09:21:05	M-marks.xml	a-s3m-M-marks.xml	curl 0	3394 bytes
```

### s3r
```
09:24:24	T-types.xml	a-s3r-T-types.xml	curl 0	319942 bytes
09:24:24	M-marks.xml	a-s3r-M-marks.xml	curl 0	3394 bytes
```

### s4m
```
09:24:47	M-marks.xml	a-s4m-M-marks.xml	curl 0	3394 bytes
```

### s4r
```
09:26:55	T-types.xml	a-s4r-T-types.xml	curl 0	332932 bytes
09:26:55	M-marks.xml	a-s4r-M-marks.xml	curl 0	3394 bytes
```

### s5m
```
09:27:23	M-marks.xml	a-s5m-M-marks.xml	curl 0	3394 bytes
```

### s5r
```
09:29:52	T-types.xml	a-s5r-T-types.xml	curl 0	345940 bytes
09:29:52	M-marks.xml	a-s5r-M-marks.xml	curl 0	3394 bytes
```

### s6m
```
09:30:10	M-marks.xml	a-s6m-M-marks.xml	curl 0	3394 bytes
```

### s6
```
09:30:18	I-MC1.xml	a-s6-I-MC1.xml	curl 0	540 bytes
```

### s6r
```
09:30:38	R-MC1.xml	a-s6r-R-MC1.xml	curl 0	94548 bytes
09:30:38	V-CN.xml	a-s6r-V-CN.xml	curl 0	52886 bytes
09:30:38	M-marks.xml	a-s6r-M-marks.xml	curl 0	3396 bytes
```

### s7m
```
09:30:58	M-marks.xml	a-s7m-M-marks.xml	curl 0	3396 bytes
```

### s7
```
09:31:07	I-MD1.xml	a-s7-I-MD1.xml	curl 0	540 bytes
```

### s7r
```
09:31:24	V-DN.xml	a-s7r-V-DN.xml	curl 0	52880 bytes
09:31:24	V-CN.xml	a-s7r-V-CN.xml	curl 0	52886 bytes
09:31:24	M-marks.xml	a-s7r-M-marks.xml	curl 0	3396 bytes
```

### s8m
```
09:31:38	M-marks.xml	a-s8m-M-marks.xml	curl 0	3396 bytes
```

### s8
```
09:32:07	I-MC1-again.xml	a-s8-I-MC1-again.xml	curl 0	540 bytes
```

### s8r
```
09:32:20	R-MC1.xml	a-s8r-R-MC1.xml	curl 0	94548 bytes
09:32:20	V-CN.xml	a-s8r-V-CN.xml	curl 0	52886 bytes
09:32:20	M-marks.xml	a-s8r-M-marks.xml	curl 0	3396 bytes
```

### s9m
```
09:32:33	M-marks.xml	a-s9m-M-marks.xml	curl 0	3396 bytes
```

### s9
```
09:32:41	I-MC1n.xml	a-s9-I-MC1n.xml	curl 0	538 bytes
```

### s9r
```
09:32:54	R-MC1.xml	a-s9r-R-MC1.xml	curl 0	94548 bytes
09:32:54	V-CN.xml	a-s9r-V-CN.xml	curl 0	52886 bytes
09:32:55	M-marks.xml	a-s9r-M-marks.xml	curl 0	3396 bytes
```

### s10
```
09:33:17	N-0010s.xml	a-s10-N-0010s.xml	curl 0	5606 bytes
09:33:17	N-0010c.xml	a-s10-N-0010c.xml	curl 0	5624 bytes
09:33:17	N-0010d.xml	a-s10-N-0010d.xml	curl 0	5620 bytes
09:33:18	N-CNc.xml	a-s10-N-CNc.xml	curl 0	5676 bytes
09:33:18	N-CNd.xml	a-s10-N-CNd.xml	curl 0	5670 bytes
09:33:18	N-DNc.xml	a-s10-N-DNc.xml	curl 0	5670 bytes
09:33:18	N-DNd.xml	a-s10-N-DNd.xml	curl 0	5668 bytes
```

### s11m
```
09:33:29	M-marks.xml	a-s11m-M-marks.xml	curl 0	3396 bytes
```

### s11
```
09:33:37	I-MC2.xml	a-s11-I-MC2.xml	curl 0	540 bytes
```

### s11r
```
09:33:52	R-MC1.xml	a-s11r-R-MC1.xml	curl 0	94548 bytes
09:33:52	V-CN.xml	a-s11r-V-CN.xml	curl 0	102760 bytes
09:33:52	N-CNc.xml	a-s11r-N-CNc.xml	curl 0	8332 bytes
09:33:53	M-marks.xml	a-s11r-M-marks.xml	curl 0	3396 bytes
```

### s12m
```
09:34:08	M-marks.xml	a-s12m-M-marks.xml	curl 0	3396 bytes
```

### s12
```
09:34:16	I-MDX.xml	a-s12-I-MDX.xml	curl 0	540 bytes
```

### s12r
```
09:34:31	V-CN.xml	a-s12r-V-CN.xml	curl 0	152614 bytes
09:34:31	N-CNc.xml	a-s12r-N-CNc.xml	curl 0	10976 bytes
09:34:31	N-CNd.xml	a-s12r-N-CNd.xml	curl 0	10962 bytes
09:34:31	M-marks.xml	a-s12r-M-marks.xml	curl 0	3398 bytes
```

### s13
```
09:37:44	M-marks.xml	a-s13-M-marks.xml	curl 0	3394 bytes
09:37:44	T-types.xml	a-s13-T-types.xml	curl 0	306946 bytes
09:37:44	V-CN.xml	a-s13-V-CN.xml	curl 0	3022 bytes
09:37:44	V-DN.xml	a-s13-V-DN.xml	curl 0	3022 bytes
```
