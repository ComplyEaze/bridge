# Row 1381-RB: results

Sheet `lab/1381-rb-sheet` at b92768d56b6e83525f13b93a1118d170b852f4ff (on the 1381-RR sheet commit 62858add). Run 2026-11-26, 05:38Z to 05:45Z, in one sitting, on the Windows lab PC. TallyPrime 7.2 in Educational Mode (title band `TallyPrime EDU` at every screenshot), pid 23160, port 9001. The book `BRIDGE STOCK CREATE LAB` (GUID bac2b5e9…) loaded by Select Company, bound, written once by W, read, shut by Shut Company, restored once from BK-S by Tally's own Restore, loaded again, read, and shut at the close; no other company loaded. Eleven sends by `send.py`, each once, in the order M1, I1, G1, U1, W, M2, I2, M3, I3, G3, U3; no key or click in Tally from M1 to W; no request between I2 and M3. No backup, no Bridge build, no fallback restore. No stop.

## The eleven send lines

| label | request | bytes | answer bytes | answer sha256 | first byte, total (s) | verdict |
|---|---|---|---|---|---|---|
| M1 | `marks.utf16le.xml` | 990 | 3,318 | `ed6554bcc9af34d808d6ee4df477803a8f22266c036aa9dd246abc909e4027e1` | 0.006263, 0.006904 | SENT-OK, exit 0 |
| I1 | `items.utf16le.xml` | 1,330 | 8,270 | `92508c8e43b9ad14e75551219291b70d5f9db326ae0299b602dc4c128de683a4` | 0.008032, 0.021387 | SENT-OK, exit 0 |
| G1 | `groups.utf16le.xml` | 1,144 | 4,170 | `b57cf4dc5311ee18bc767d9a3a8f6fa1220cdae9f72461066a85964b5d84602a` | 0.021909, 0.024616 | SENT-OK, exit 0 |
| U1 | `units.utf16le.xml` | 1,174 | 4,040 | `cb9d693327fb6b8b27c9449ee9f841cc12f8a4afe2598b38692a0f825f0d4f39` | 0.004105, 0.004735 | SENT-OK, exit 0 |
| W | `create.utf16le.xml` | 3,646 | 538 | `1b72152a8fd20ccb9366277864bf32357b8891e536cde2a54992adaa60a97010` | 0.448700, 0.449306 | SENT-OK, exit 0 |
| M2 | `marks.utf16le.xml` | 990 | 3,318 | `4e9f77ceaba1315dbbe2c340da3bdf30d6a8f9fca36145e24570f7fb53760f81` | 0.004041, 0.004617 | SENT-OK, exit 0 |
| I2 | `items.utf16le.xml` | 1,330 | 10,758 | `2ae12ef647f24b9645231b868f733f978b09d3a8508aab5bf6dfa2564ccbffbd` | 0.006135, 0.006835 | SENT-OK, exit 0 |
| M3 | `marks.utf16le.xml` | 990 | 3,318 | `ed6554bcc9af34d808d6ee4df477803a8f22266c036aa9dd246abc909e4027e1` | 0.003764, 0.004284 | SENT-OK, exit 0 |
| I3 | `items.utf16le.xml` | 1,330 | 8,270 | `92508c8e43b9ad14e75551219291b70d5f9db326ae0299b602dc4c128de683a4` | 0.007307, 0.007964 | SENT-OK, exit 0 |
| G3 | `groups.utf16le.xml` | 1,144 | 4,170 | `b57cf4dc5311ee18bc767d9a3a8f6fa1220cdae9f72461066a85964b5d84602a` | 0.008166, 0.018440 | SENT-OK, exit 0 |
| U3 | `units.utf16le.xml` | 1,174 | 4,040 | `cb9d693327fb6b8b27c9449ee9f841cc12f8a4afe2598b38692a0f825f0d4f39` | 0.006043, 0.007578 | SENT-OK, exit 0 |

Each: curl exit 0, status 200, CONTENT-LENGTH equal to the answer's bytes. Sent at 05:39:58.343Z (M1), 05:40:07.950Z (I1), 05:40:12.126Z (G1), 05:40:16.353Z (U1), 05:40:32.421Z (W), 05:40:47.294Z (M2), 05:40:55.524Z (I2), 05:43:33.900Z (M3), 05:43:42.301Z (I3), 05:43:47.293Z (G3), 05:43:51.255Z (U3). `evidence/send-log.txt` holds the eleven lines whole.

## The bind

- `measure.py state M1` (`evidence/m1-state.txt`): one COMPANY row, the book's GUID, ALTMSTID 210, ALTVCHID absent: `BOOK STATE MET`, exit 0.
- `measure.py bind` (`evidence/bind.txt`): `BIND MET: M1 one COMPANY row, the book's, ALTMSTID 210, no ALTVCHID; I1, G1 and U1 byte-equal to the pins`, exit 0; M1 byte-equal to `stock-create-lab-marks.utf16le.xml` (a measurement).
- `restore.py precheck`, the book loaded, after the bind (`evidence/r-precheck.txt`): `PRECHECK MET`: BK-S and the fallback one file each, 24,219 B, sha256 equal to recorded, the same company number in both; the company's data folder present and listed, 21 entries; exit 0.

## The write

- The UTC time before the send line: 05:40:32Z (the step 3 notes line). W sent at 05:40:32.421Z, the start marker written before curl; answered in 0.449306 s; `SENT-OK`, exit 0.
- Straight after the send line returned: `s3-write.jpg`, saved at 05:40:35Z: the Gateway as at step 1, the book the one company, title band EDU, no dialog or message.
- `measure.py write` (`evidence/w-grade.txt`): 538 B, byte-equal to the licensed answer `stock-create-lab-create-answer.utf16le.xml`: yes; CREATED `'1'`, ALTERED `'0'`, DELETED `'0'`, LASTVCHID `'0'`, LASTMID `'0'`, COMBINED `'0'`, IGNORED `'0'`, ERRORS `'0'`, CANCELLED `'0'`, EXCEPTIONS `'0'`; no LINEERROR: `WRITE ADMITTED`, exit 0.
- W's answer whole (`answers/a-W.xml`, UTF-16LE, shown decoded):

```xml
<RESPONSE>
 <CREATED>1</CREATED>
 <ALTERED>0</ALTERED>
 <DELETED>0</DELETED>
 <LASTVCHID>0</LASTVCHID>
 <LASTMID>0</LASTMID>
 <COMBINED>0</COMBINED>
 <IGNORED>0</IGNORED>
 <ERRORS>0</ERRORS>
 <CANCELLED>0</CANCELLED>
 <EXCEPTIONS>0</EXCEPTIONS>
</RESPONSE>
```

## The aim

- `measure.py state M2` (`evidence/m2-state.txt`): one COMPANY row, the book's GUID, ALTMSTID 211, ALTVCHID absent: `BOOK STATE MET`, exit 0.
- `measure.py aim` (`evidence/aim.txt`): I2 two STOCKITEM rows, `SC Copied Goods` (GUID bac2b5e9…-000000d1) and `SC Sibling Goods` (bac2b5e9…-000000d0), each bound to the book; I2 byte-equal to the licensed read `stock-create-lab-items-after-plain-create.utf16le.xml`: yes; `AIM SHOWN: 'SC Copied Goods' in I2, bound to the book`, exit 0.

## The restore

- The shut, by Shut Company: no renewal dialog and no License screen; `s5-select.jpg` (05:41:21Z): Select Company, no company loaded, EDU.
- `restore.py plan` (`evidence/r-plan.txt`): `PLAN WRITTEN`: one file, 24,219 B, sha256 `ecc8d4028b88b763e5a8a22014bc699b07db68c3eb32cff3c6c92052b373b0f4` (equal to recorded), modified 2026-11-19 at 19:40:11 local; the company's data folder 19 entries; exit 0.
- On Tally's screen (the step 5 notes line; times from the screenshots' own save times, as the sheet allows): first click (Y: Data) at about 05:41:30Z; Backup & Restore; Restore. The Restore Data Path read Tally's data folder (`s5-restore-zoom.jpg`). Specify Path (click, Enter), the BK-S folder typed (zoom: exact), Enter. `s5-list.jpg` (05:42:02Z): `BRIDGE STOCK CREATE LAB` alone, with the plan's number and `19-Nov-26 (7:40 pm)`. Enter on that row; one click on End of List. **"Accept ?"** (Yes or No, `s5-accept.jpg`, 05:42:20Z); `y`. **The folder-exists prompt** (`s5-replace.jpg`, 05:42:30Z): "Company data folder (`<company number>`) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder.", R: Replace / S: Skip, the number the plan's (`s5-replace-zoom.jpg`); `r` at about 05:42:40Z. `s5-after.jpg`, taken 5 s after the `r`: Select Company, no company loaded, EDU, no dialog. No other prompt; no licence, activation, renewal or Educational-Mode screen; no Escape.
- `restore.py after`, back at Select Company before the load (`evidence/r-after.txt`, whole there): `AFTER RECORDED`: the source unchanged; the company's data folder 19 entries before, 15 after; 1 added (`TMESSAGE.TSF`), 5 removed (`CmpSchMetaData.1800`, `Index.1800`, `TCMPSCHACCESS.TSF`, `TCMPSCHSTATE.TSF`, `TCMPSCHUPDATE.TSF`), 14 changed, 0 unchanged; every changed entry's modified time 2026-11-26 at 11:12:40 local (05:42:40Z), the Replace; `Manager.1800` from 194,048 B to 193,024 B, every other changed entry's size the same; exit 0.

## The reads after the restore and the compute

- The load: `s6-window.jpg` and `s6-gateway.jpg` (05:43:26Z): the book the one company, EDU, no dialog; period 1-Apr-25 to 31-Mar-26, current date Tuesday, 1-Apr-2025, as at step 1.
- `measure.py state M3` (`evidence/m3-state.txt`): one COMPANY row, the book's GUID, ALTMSTID 210, ALTVCHID absent: `BOOK STATE MET`, exit 0.
- `measure.py all local/gen/fx` (`evidence/measure-all.txt`, whole there): every label read; M1, M3 one COMPANY row; I1, I3 one STOCKITEM row `SC Sibling Goods`; I2 two rows; G1, G3 one STOCKGROUP row `SC Goods Group`; U1, U3 one UNIT row `Nos`; W `WRITE ADMITTED`; the M1, M2 and M3 states met at ALTMSTID 210, 211 and 210; the bind met; the aim yes and byte-equal yes; M3, I3, G3 and U3 against the pins byte-equal yes, rows equal yes; `RESTORE UNDID THE WRITE: yes (M3, I3, G3 and U3 byte-equal to the pins: 4 of 4)`; `ALL 11 ANSWERS READ`, exit 0.

## The questions

- **Q1: yes, for this book in this sitting.** In Educational Mode the stock-item Create sent as a file was admitted: `WRITE ADMITTED`, W's answer byte-equal to the licensed answer of row 1381-R1, M2's state met at ALTMSTID 211, and `AIM SHOWN`. No dialog or message came on Tally's screen after it.
- **Q2: yes.** Between the bind (M1) and the aim (M2) the master mark moved from 210 to 211, and I2 shows `SC Copied Goods` beside `SC Sibling Goods`, bound to the book, byte-equal to the licensed read after the same write.
- **Q3: yes, byte for byte.** After Tally's own Restore of BK-S, M3, I3, G3 and U3 are byte-equal to the pins (`RESTORE UNDID THE WRITE: yes`, 4 of 4), M3 at ALTMSTID 210. The restore's only prompts were "Accept ?" and the Replace prompt; Tally was back at Select Company within 5 s of the Replace; the data folder shows 14 entries rewritten at the Replace, `Manager.1800` 1,024 B shorter, `Index.1800` (stamped at W's time) removed.

Each answer is one book, one write, one backup, one release, one sitting, in Educational Mode: partial, and recorded so. What the book holds: BK-S restored; the book at BK-S's state.

## Steps and EXPECT

| step | EXPECT | met |
|---|---|---|
| 0 | `made`, exit 0; the screenshot straight after: Select Company, no company loaded, EDU, no dialog; fetch exit 0; copy `exit 0, 0`; `--fresh` four `OK`, exit 0; pins `failed 0`; self-checks 58, 65 and 44 right and check_sheet three `OK`, each exit 0; no `bridge_mcp.exe`; one `tally.exe`, pid 23160; `:9001` LISTENING on that pid and 40 LISTENING lines in all; GET `/status` status 200, 51 bytes, exit 0 | met |
| 1 | the book the one row left in the filter; the Gateway with the book as the one company, EDU, no dialog; period 1-Apr-25 to 31-Mar-26 and Tuesday 1-Apr-2025; M1 `SENT-OK`; `BOOK STATE MET`; I1, G1, U1 `SENT-OK`; `BIND MET`, each exit 0 | met |
| 2 | `PRECHECK MET`, exit 0 | met |
| 3 | W `SENT-OK`, exit 0; the screenshot straight after: the Gateway, EDU, no dialog; `WRITE ADMITTED`, exit 0 | met |
| 4 | M2 `SENT-OK`; `BOOK STATE MET` at ALTMSTID 211; I2 `SENT-OK`; `AIM SHOWN`, I2 byte-equal to the licensed read, each exit 0 | met |
| 5 | the shut; Select Company, EDU; `PLAN WRITTEN`, exit 0; the Restore Data Path Tally's data folder; the list the book alone with the plan's number and `19-Nov-26 (7:40 pm)`; "Accept ?", then the Replace prompt naming the plan's number; Select Company within 10 minutes, EDU; `AFTER RECORDED`, exit 0 | met: no licence pair at the shut; back at Select Company within 5 s (deviation 2) |
| 6 | the book in the filter; the whole window and the Gateway: the book the one company, EDU, no dialog; M3 `SENT-OK`; `BOOK STATE MET` at ALTMSTID 210; I3, G3, U3 `SENT-OK`; the screenshot after U3: the Gateway, EDU, no dialog; `RESTORE UNDID THE WRITE: yes (… 4 of 4)` and `ALL 11 ANSWERS READ`, exit 0 | met (deviation 3) |
| 7 | the shut; Select Company, no company loaded, EDU; the same `tally.exe` pid 23160 owning `:9001`; GET `/status` 200, exit 0; `check_sheet.py` three `OK`, exit 0; the listing exit 0 | met: no licence pair; 40 LISTENING lines; `/status` 200, 51 bytes; check_sheet at 05:44:29Z; the listing at 05:44:35Z, 414 files under `local/`, exit 0 |

The keys. Each load: one click in the Select Company filter box; `BRIDGE`, then a space and `STOCK`, then a space and `CREATE` (one row left); Enter. Each shut: one click on the Gateway's left panel; Alt+K; one click on SHut; Enter on the one row. The restore: as in The restore above.

## Deviations

1. Beyond the sheet's named screenshots, four more were kept: `s1-gateway-zoom.jpg` (a zoom of the Gateway's left panel in the same action as `s1-gateway.jpg`), `s5-data-menu.jpg` (the open Data menu, whose save time gives the first click), and `s5-restore.jpg` with its zoom `s5-restore-zoom.jpg` (the Restore screen and its Restore Data Path). `s6-gateway.jpg` is a zoom of the Gateway's left panel taken in the same action as `s6-window.jpg`, as the sheet allows.
2. `s5-replace-zoom.jpg` (taken before the `r`) and `s5-after.jpg` (taken 5 s after it) were saved together at 05:42:45Z, at the end of one screen-tool action; the Replace's time, about 05:42:40Z, is taken from the data folder's modified times (11:12:40 local).
3. The screenshot after U3 (`s6-after-u3.jpg`, 05:43:55Z) and the `measure.py all` line were issued together, so the compute may have begun before the screenshot was saved; the compute reads files only and sends nothing.

## Observations, not findings

- The company's data folder held 21 entries with the book loaded (precheck, after the bind) and 19 once it was shut; row 1381-RR's had 18 after its shut. The extra entry is `Index.1800`, stamped 11:10:32 local (05:40:32Z, W's send), which the restore removed with the four entries row 1381-RR's restore removed.
- W's answer carried no GUID and no LINEERROR; its first byte came after 0.4487 s, against under 0.03 s for every read.
- The new item's GUID in I2 is the book's GUID with the suffix `-000000d1`, the next after the sibling's `-000000d0`.
- `/status` answered with the same 51 bytes at step 0 and at the close (`TallyPrime Server is Running`).

## Not covered

As the sheet's Not covered: Bridge itself refusing the recheck (its masters runtime refuses Educational Mode), a backup in Educational Mode, row 1381-R1's marked Create, an alias, any other write, any restore of row 1381-O's backup-1, a licensed write or restore, a Bridge build, other books, releases and sittings, and transport failure.

## The record

`evidence/` (every helper's output with its exit code, the send log, the bind, the grade, the aim, the precheck, plan and after lines, the three state files, the compute, the two `/status` captures, `s7-listing.txt` and the notes), `answers/` (all eleven answers exactly as received, with their headers, transport records and start markers). Kept on the lab PC, listed in `kept-local.txt` by size and sha256: `precheck.txt`, `restore-plan.txt` and the two data-folder listings (they carry the company number or Tally's data folder), the screenshots, the `tasklist` and `netstat` output, master's fixtures in `local/gen/fx/` (on master already), the self-tests' scratch, the stage and scan helpers and the commit message. Text files had lab and clone paths and the gate's desk words replaced; `substitutions.txt` counts every replacement.
