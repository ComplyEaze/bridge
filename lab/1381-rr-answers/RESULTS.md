# Row 1381-RR: results

Sheet `lab/1381-rr-sheet` at 62858adda5a61cf7b0e94c54f598bf4be19b687d (on the 1381-RA sheet commit 30f5c820). Run 2026-11-26, 04:35Z to 04:41Z, in one sitting, on the Windows lab PC. TallyPrime 7.2 in Educational Mode (title band `TallyPrime EDU` at every screenshot), pid 23160, port 9001. The book `BRIDGE STOCK CREATE LAB` (GUID bac2b5e9…) loaded by Select Company, shut by Shut Company, restored once from BK-S by Tally's own Restore, loaded again and shut at the close; no other company loaded. Five reads by `send.py`, each once, in the order M1, M2, I, G, U; nothing sent to Tally between M1 and M2. No write, no backup, no Bridge build, no fallback restore. No stop.

## The five send lines

| label | request | bytes | answer bytes | answer sha256 | first byte, total (s) | verdict |
|---|---|---|---|---|---|---|
| M1 | `marks.utf16le.xml` | 990 | 3,318 | `ed6554bcc9af34d808d6ee4df477803a8f22266c036aa9dd246abc909e4027e1` | 0.003734, 0.004209 | SENT-OK, exit 0 |
| M2 | `marks.utf16le.xml` | 990 | 3,318 | `ed6554bcc9af34d808d6ee4df477803a8f22266c036aa9dd246abc909e4027e1` | 0.004433, 0.005232 | SENT-OK, exit 0 |
| I | `items.utf16le.xml` | 1,330 | 8,270 | `92508c8e43b9ad14e75551219291b70d5f9db326ae0299b602dc4c128de683a4` | 0.008272, 0.008987 | SENT-OK, exit 0 |
| G | `groups.utf16le.xml` | 1,144 | 4,170 | `b57cf4dc5311ee18bc767d9a3a8f6fa1220cdae9f72461066a85964b5d84602a` | 0.006770, 0.007412 | SENT-OK, exit 0 |
| U | `units.utf16le.xml` | 1,174 | 4,040 | `cb9d693327fb6b8b27c9449ee9f841cc12f8a4afe2598b38692a0f825f0d4f39` | 0.004663, 0.005478 | SENT-OK, exit 0 |

Each: curl exit 0, status 200, CONTENT-LENGTH equal to the answer's bytes. Sent at 04:36:45.766Z (M1), 04:39:32.330Z (M2), 04:39:41.530Z (I), 04:39:46.700Z (G), 04:39:51.397Z (U). `evidence/send-log.txt` holds the five lines whole.

## The restore

- `restore.py precheck`, the book loaded, after M1's state (`evidence/r-precheck.txt`): `PRECHECK MET`: BK-S and the fallback one file each, 24,219 B, sha256 equal to recorded, the same company number in both; the company's data folder present and listed, 21 entries; exit 0 at 04:36:55Z.
- `restore.py plan`, after the shut (`evidence/r-plan.txt`): `PLAN WRITTEN`: one file, 24,219 B, sha256 `ecc8d4028b88b763e5a8a22014bc699b07db68c3eb32cff3c6c92052b373b0f4` (equal to recorded), modified 2026-11-19 at 19:40:11 local; the company's data folder 18 entries; exit 0 at 04:37:25Z.
- On Tally's screen (the step 4 notes line): first click (Y: Data) at about 04:37:28Z; Backup & Restore; Restore. The Restore Data Path read Tally's data folder (zoom). Specify Path, the BK-S folder typed (zoom: exact), Enter. The list showed `BRIDGE STOCK CREATE LAB` alone, with the plan's number and `19-Nov-26 (7:40 pm)`. Enter on that row; one click on End of List alone. **"Accept ?"** (Yes or No) showed; `y`. **The folder-exists prompt**, "Company data folder (`<company number>`) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder.", R: Replace / S: Skip, the number the plan's (zoomed); `r` at about 04:38:47Z. Back at Select Company, no company loaded, title band EDU, no dialog, by 04:38:52Z: about 5 s after the Replace. No other prompt; no licence, activation, renewal or Educational-Mode screen; no Escape.
- `restore.py after`, back at Select Company before the load (`evidence/r-after.txt`, whole there): `AFTER RECORDED`: the source unchanged; the company's data folder 18 entries before, 15 after; 1 added (`TMESSAGE.TSF`), 4 removed (`CmpSchMetaData.1800`, `TCMPSCHACCESS.TSF`, `TCMPSCHSTATE.TSF`, `TCMPSCHUPDATE.TSF`), 14 changed, 0 unchanged; every changed entry's size the same and its modified time 2026-11-26 at 10:08:48 local (04:38:48Z), the Replace; exit 0.

## The state and the compute

- `measure.py state M1` (`evidence/m1-state.txt`): one COMPANY row, the book's GUID, ALTMSTID 210, ALTVCHID absent: `BOOK STATE MET`, exit 0.
- `measure.py state M2` (`evidence/m2-state.txt`): the same: `BOOK STATE MET`, exit 0.
- `measure.py all` (`evidence/measure-all.txt`, whole there): every label read; I one STOCKITEM row `SC Sibling Goods`, G one STOCKGROUP row `SC Goods Group`, U one UNIT row `Nos`, each bound to the book (1 of 1); M1 against M2 byte-equal yes, rows equal yes; I, G and U against row 1381-RA's `a-I1.xml`, `a-G1.xml` and `a-U1.xml` byte-equal yes, rows equal yes; `ALL 5 ANSWERS READ`, exit 0.

## The questions

- **Q1: yes, for this book in this sitting.** The restore of BK-S completed in Educational Mode with "Accept ?" and the Replace prompt as its only prompts, both screenshotted; no licence, activation, renewal or Educational-Mode screen came; Tally was back at Select Company about 5 s after the Replace; `AFTER RECORDED` with 1 entry added and 14 changed (all stamped at the Replace), 4 removed.
- **Q2: yes.** M2's state is M1's (one company row, the book's GUID, ALTMSTID 210, no ALTVCHID), and M2 is byte-equal to M1. As the sheet says, this alone cannot show the restore did anything, since the book was already at BK-S's state; the listing difference (Q4) is that evidence.
- **Q3: yes, byte for byte.** I, G and U are byte-equal to row 1381-RA's I1, G1 and U1.
- **Q4.** The company's data folder: 21 entries with the book loaded (precheck), 18 after the shut (plan), 15 after the restore. The restore removed `CmpSchMetaData.1800` and the three `TCMPSCH*.TSF` files, added `TMESSAGE.TSF`, and rewrote the other 14 entries with their sizes unchanged and the Replace's time. After it the title band read `TallyPrime EDU`; the Gateway (step 5 and step 6) showed the book as the one company, No Vouchers Entered, period 1-Apr-25 to 31-Mar-26, current date Tuesday, 1-Apr-2025, as at step 1 and as lab 1381-RA recorded.

Each answer is one book, one backup, one release, one sitting, in Educational Mode: partial, and recorded so.

## Steps and EXPECT

| step | EXPECT | met |
|---|---|---|
| 0 | `made`, exit 0; the screenshot straight after: Select Company, no company loaded, EDU, no dialog; fetch exit 0; copy `exit 0, 0`; `--fresh` four `OK`, exit 0; `present 851e2545…`; RA's answers `exit 0, 0`; self-checks 28, 30 and 34 right and check_sheet three `OK`, each exit 0; no `bridge_mcp.exe`; one `tally.exe`, pid 23160; `:9001` LISTENING on that pid and 40 LISTENING lines in all; GET `/status` status 200, 51 bytes, exit 0 | met |
| 1 | the book the one row left in the filter; the Gateway with the book as the one company, EDU, no dialog; period 1-Apr-25 to 31-Mar-26 and Tuesday 1-Apr-2025 | met |
| 2 | M1 `VERDICT SENT-OK`, exit 0; `BOOK STATE MET`, exit 0; `PRECHECK MET`, exit 0 | met |
| 3 | the shut; Select Company, no company loaded, EDU | met: no renewal dialog and no License screen came, so no licence click was made |
| 4 | `PLAN WRITTEN`, exit 0; the Restore Data Path Tally's data folder; the list the book alone with the plan's number and `19-Nov-26 (7:40 pm)`; "Accept ?", then the Replace prompt naming the plan's number; Select Company within 10 minutes, EDU; `AFTER RECORDED`, exit 0 | met (deviation 2) |
| 5 | the book in the filter; the whole window and the Gateway: the book the one company, EDU, no dialog; period and date as at step 1 | met (deviation 1) |
| 6 | M2 `SENT-OK`, exit 0; M2's state `BOOK STATE MET`, exit 0; I, G, U `SENT-OK`, exit 0; the screenshot the Gateway as at step 5, EDU, no dialog | met |
| 7 | `ALL 5 ANSWERS READ`, exit 0 | met |
| 8 | the shut; Select Company, no company loaded, EDU; the same `tally.exe` pid 23160 owning `:9001`; GET `/status` 200, exit 0; `check_sheet.py` three `OK`, exit 0; the listing exit 0 | met: no licence pair; 40 LISTENING lines; `/status` 200, 51 bytes; check_sheet at 04:40:30Z; the listing at 04:40:36Z, 229 files under `local/`, exit 0 |

The keys. Each load: one click in the Select Company filter box; `BRIDGE`, then a space and `STOCK`, then a space and `CREATE` (one row left); Enter. Each shut: one click on the Gateway's left panel; Alt+K; one click on SHut; Enter on the one row. The restore: as in The restore above.

## Deviations

1. Step 5's Gateway screenshot `s5-gateway.jpg` is a zoom on the Gateway's left panel taken in the same action as the whole-window screenshot `s5-window.jpg`, not a second full screenshot; both show the book as the one company, EDU, no dialog.
2. Step 4's notes line gives the first click (Y: Data) as about 04:37:35Z. The screenshot of the open Data menu was saved at 04:37:30Z, so the first click was at about 04:37:28Z; the notes end before the listing that hashes them, so the correction is here. The zoom on the Replace prompt's number is kept as its own file, `s4-replace-zoom.jpg`, beside `s4-replace.jpg`.
3. `s6-after-u.jpg` was taken at the screen tool's 0.6 scale; it shows the Gateway as at step 5, EDU, no dialog.

## Observations, not findings

- The Select Company filter showed the typed spaces this time (`BRIDGE STOCK CREATE`), unlike row 1381-RA's sitting.
- The company's data folder held 21 entries with the book loaded and 18 once it was shut: three side files went at the shut. The restore then removed four more entries and added one.
- `/status` at the close answered with the same 51 bytes as at step 0 (`TallyPrime Server is Running`).

## Not covered

As the sheet's Not covered: a restore that undoes a change in Educational Mode (row 1381-RB), a backup in Educational Mode, a write, any restore of row 1381-O's backup-1, a licensed restore, the count read, a Bridge build, other books, releases and sittings, and transport failure.

## The record

`evidence/` (every helper's output with its exit code, the send log, the precheck, plan and after lines, the two state files, the compute, the two `/status` captures, `s8-listing.txt` and the notes), `answers/` (M1, M2, I, G and U exactly as received, with their headers and transport records). Kept on the lab PC, listed in `kept-local.txt` by size and sha256: `precheck.txt`, `restore-plan.txt` and the two data-folder listings (they carry the company number or Tally's data folder), the screenshots, the `tasklist` and `netstat` output, row 1381-RA's answers in `local/gen/ra/`, the self-tests' scratch, the stage and scan helpers and the commit message. Text files had lab and clone paths and the gate's desk words replaced; `substitutions.txt` counts every replacement.
