# Part W (W1, W2, W3) with P0 to A3, M, O and Close: results

First line: **Every EXPECT of the W row is met (verified), with three notes for the lead: the proxy logged ConnectionAbortedError (WinError 10053) where the sheet names a broken pipe at W3, it logged a second error (ConnectionResetError, WinError 10054) when the app was closed, and the mailing answer at M2 is 334 B longer than row 2214's L6 plus the probe's row.** The app's workbook export read the mailing details whole at W1 (master mark 4400) and W2 (30,125,084 B). At W3 (37,647,244 B, past 32 MiB) the app read the book again without them and wrote the note into cell C3. The book was restored by C6 at the close, and the marks are byte-equal to S0's.

Sheet `lab/2074-2214-export-cut` at `7a49b5c1`, kit 11 of 11 OK by its `SHA256.txt` at P0 and again at the close. All times are UTC on 2026-11-18. Every number is read from its file here, and `evidence/notes.txt` has each step.

**Build, on the run lead's word of 2026-11-18** (not the merge-queue run the sheet names): CI run <CI run id, as reported> (scheduled, master, success) for commit `a83ee02da443fb1e2256919b18b4a5fb84056781` (2026-11-12 07:29Z). Artifact `unsigned-bundle-smoke-windows-latest`, GitHub digest `sha256:4f6e91b082d5fde228aded36713e65dcb0f65f36a7fb7f13bb03730aa8a72734`; the zip on the PC is equal. The installed desktop app is 0.7.0: `bridge.exe` sha256 `8c1a640d4fb8a8e2586683beb90bd421776bae07908bf03f351906230e53a8ed`, `bridge_mcp.exe` `7086f1d0d7430e7dc1fefd6f69c0d4a14d717a2e06477b32935a1d3aa4fee1b8`, both read from the real install path (outside the package view) before W1 and again at the close. The server for every tool read is that bundle's `bridge_mcp.exe` (`7086f1d0...`, equal to the bundle's `binary_sha256`), server version 0.7.0. TallyPrime 7.2, edition as recorded. Book BRIDGE BATCH LAB, synthetic.

**The build and the sheet's lines.** `a83ee02d` holds `d214d6a0`, the commit the sheet's file lines are at. The behaviour the W EXPECTs rest on is in it: the 32 MiB note (`party_ledger_master.rs:151` and `:185` here, `:146` and `:180` at `d214d6a0`), `MailingRetryHeldByDrain`, "Balances as of", and Downloads by `download_dir` (`commands.rs:2803`, the sheet's `:2085`). The Outstandings button reads "Ledger master + group subtotal trace" (`OutstandingsScreen.tsx:714`); it is the sheet's "Ledger master + group subtotals". No EXPECT needed a change merged after 2026-11-12.

## Before the sitting and Part A (verified)

| step | EXPECT | measured | grade |
|---|---|---|---|
| P0 | THE COMPANY alone, marks at rest | BRIDGE BATCH LAB loaded alone; one tally.exe on 9001; GET /status 200; MARKS master mark 224, voucher mark 87, answer `566be20d...`, byte-equal to 2026-11-09's | verified |
| make_creates | equal to `requests-manifest.txt` | 201 lines equal; only the line ending differs (CRLF from the Windows print) | verified |
| S0 | BK by C5, marks after | one file in the new empty `bk`, 57,180 B, sha256 `e744aa9dafc42317a88a8ea204431201bba0ba7b96645975c24ceadef0936550` (kept on the lab PC), 08:47:45Z; path set by Specify Path, one company; MARKS after byte-equal to P0's | verified |
| B5 | no Bridge process of ours | none; the desktop app was not running; 9002 and 9003 not listening | verified |
| start | proxy, then server; `list_companies` | proxy on 127.0.0.1:9002 to 9001, up to the close; server 0.7.0, `7086f1d0...`; tools/list saved (kept local); THE COMPANY alone, `verified_tuple` | verified |
| R1 | STANDARD twice, 120 s apart: total 11, marks 224/87 | 08:49:33Z and 08:51:33Z: total 11 both, marks 224/87 both; master posts held 21/17 ms and 46/33 ms; post threshold max(92, 2,046) = 2,046 ms; drain record `{"state":"clear"}` | verified |
| A1 | create-probe, create-000: CREATED 1 each | create-probe CREATED 1 (master mark 225); create-000 CREATED 1 (226), the capture after it byte-equal to the reference Gateway | verified |
| A2 | MAILING: total 13; Lab Mail Load 000 five entries, oldest first | total 13; entries 20260401 to 20260405, 1 to 5 Lab Street, Lab Area, 560001, Karnataka; Lab Mark Probe none | verified |
| A3 | alterations to a master mark of at least 4,400 | 4174 alterations (alter-a and alter-b in turn), 08:52:38Z to 09:10:15Z, 1056.4 s; every one ALTERED 1, others 0, no LINEERROR; last mark m = 4400, voucher mark 87 | verified |

## Part W1: the workbook past the mark (verified)

The app was started at 09:10:42Z from its real path: process `bridge.exe` at `<local app data>\Bridge\bridge.exe`, file sha256 `8c1a640d...`, as the run lead required. Its endpoint read 127.0.0.1:9002 already; Check Tally showed it reachable and offered THE COMPANY alone, which was chosen. The export was pressed on the Overview screen at 09:12:13Z, with the As of field at the host's date (18-11-2026).

| EXPECT | measured | grade |
|---|---|---|
| the master post a mailing one, after a count read | proxy log (`proxy/proxy.jsonl`, lines 601 on): a count read (List of Ledgers, name, GUID and parent only, 1974 B) twice, then the mailing master post (1784 B, `LEDMAILINGDETAILS.LIST`) answered whole, 36,444 B, twice; no master post with the standard fetch; no error | verified |
| C4 empty; C3 no failed-read note | both empty | verified |
| Lab Mail Load 000's Mailing cells | "5 Lab Street, Lab Area", 560001, Karnataka, from 20260405, "in force" | verified |
| Lab Mark Probe's status | "no mailing details" (Alter ID 4400) | verified |
| record time, sha256, "Balances as of" | 09:12:14Z; `party-ledger-master-bridge-batch-lab-20260831.xlsx`, 10,564 B, sha256 `e3617851a5c1b9dbc6508c5fbfdc9b6209e462892752810e9efc495a16c0483c`; Balances as of 20260831 (the As of field showed the host's date; the workbook and its name carry 20260831); 13 ledger rows | recorded |

## Part M and W2 (verified)

| step | EXPECT | measured | grade |
|---|---|---|---|
| M1 | create-001 to create-160; MARKS m + 160 | 160 of 160 CREATED 1, no LINEERROR, 81.2 s; master mark 4560, voucher mark 87 | verified |
| M2 STANDARD | total 173 | total 173 at 09:15:16Z; master posts 1736 B answered 326,894 B, held 57/37 ms | verified |
| M2 MAILING | about 30 MB, 32,005 entries; bytes as L6 plus the probe's row | total 173 at 09:15:23Z, served in three pages from one read; the mailing master post answered 30,125,084 B twice, 2.360 s and 1.015 s; dated entries: Lab Mail Load 000 five and 001 to 160 two hundred each (32,005), plus one on each of seven seed ledgers (32,012 in the answer). **Note:** 2,554 B more than L6's 30,122,530 B; the probe's ledger element in this answer is 2,220 B, so 334 B more than the probe's row | verified, with the note |
| W2 | mailing post whole; C4 empty; no failed-read note; 173 rows; Lab Mail Load 160's latest entry on or before the balance date, "in force" | 09:16:39Z: count read, then the mailing master post answered whole, 30,125,084 B, 2.906 s and 1.891 s; workbook `...-20260831-2.xlsx`, 26,790 B, sha256 `c162d00a83aca314bf200794d95912cdaf93563d2cb18da71979828df820c6b2`; C3 and C4 empty; 173 rows; Lab Mail Load 160: "153 Lab Street, Lab Area", 560001, Karnataka, from 20260831, "in force" | verified |

## Part O and W3 (verified, with the proxy note)

| step | EXPECT | measured | grade |
|---|---|---|---|
| O1 | create-161 to create-200; MARKS last + 40, voucher mark unmoved | 40 of 40 CREATED 1, 18.2 s; master mark 4600, voucher mark 87 | verified |
| O2 | drain record clear | `{"state":"clear"}`, 17 B (copy in `debt/`) | verified |
| W3 order | the mailing post first, then status sends if a drain is owed, then a standard master post | 09:17:55Z: count read; the mailing master post (conn 5, 1784 B), Tally's first byte 3.4 s after it, 37,647,244 B in all; the app closed that connection; a new connection sent three status gets 2 s apart (drain owed), then the standard master post (1736 B, answered 401,694 B) | verified |
| W3 time | from the mailing post to the standard post, against 60 s | 8.375 s | verified |
| W3 workbook | 215 ledgers, every Mailing status "not observed" | `...-20260831-3.xlsx`, 28,600 B, sha256 `9e92c8c2efc6ed66f018598c9be1ec78fe9c9d105d70ebd097eb054cf9f02704`; **213** ledgers, every Mailing status "not observed" (a=213, b=213). The sheet's 215 counts the two ledgers of Parts F and X, which ran in other sittings and were not part of this one | verified |
| W3 note | C3 holds "Tally's answer to that read passed the 32 MiB limit, so this workbook was read again without them"; C4 empty | C3: "The mailing details were asked for, and Tally's answer to that read passed the 32 MiB limit, so this workbook was read again without them. The Mailing columns are empty and their status says not observed: that is not the same as a party having no mailing address. Read the address in Tally." C4 empty | verified |
| W3 app | the app's notes equal to the workbook's | the app's notice reads the same sentence, word for word | verified |
| W3 mark | master mark unmoved | MARKS after: master mark 4600, voucher mark 87; drain record clear | verified |

**Proxy note (as logged, not explained):** on the W3 mailing post, `proxy_error` conn 5, `ConnectionAbortedError`, "[WinError 10053] An established connection was aborted by the software in your host machine". The sheet names a broken pipe here. A second one at 09:19:21Z, conn 6, `ConnectionResetError`, "[WinError 10054] An existing connection was forcibly closed by the remote host", when the app's window was closed at Close 1. No `bridge_cut` and no `tally_closed` in the whole log.

## Close (verified)

- Close 1: reads only. The app was closed by its window's close button, the server stopped by its stop file (exit code 0), and the proxy was stopped. The log was parsed whole: 453 requests forwarded and answered on 6 connections, the two errors above, nothing else.
- Close 2: the kit 11 of 11, the 201 request files equal to the manifest, the server exe `7086f1d0...` and the installed app's two files (`8c1a640d...`, `7086f1d0...`) unchanged; BK unchanged.
- Close 3, on the run lead's grant (words given in advance): BK-2074-2214 restored by C6. Before it: the backup re-hashed, `evidence/restore-plan.txt` written, the company's data folder listed (kept on the lab PC). THE COMPANY shut by Company > Shut. The Restore list read BRIDGE BATCH LAB, its number, 18-Nov-26 (2:17 pm), the backup meant. Only "Accept ?" (Yes) and "Company data folder (<company number>) already exists ..." (Replace) came up. The folder held 24 files before and 17 after: seven side files were absent after the restore, the pattern seen at earlier restores here.
- Close 4: MARKS master mark 224, voucher mark 87, 3,394 B, `566be20d...`, byte-equal to S0's. `ledger_masters` basic through a new server on port 9001 (`run3/`): total 11.
- C10: shut by Company > Shut; Tally at Select Company, no company loaded, no dialog; one tally.exe owns 9001, nothing on 9002; GET /status 200; Tally's current date 21-Aug-2026, unchanged by this row. No XML request after the shut.

## Files

- `run2/answers/`: every tool answer (company_number redacted by a JSON parser) except tools/list and the three MAILING pages of M2, each over 2,000,000 B (kept on the lab PC by sha256 in `kept-local.txt`); `run2/client-log.txt`, `launch-env.json`, the server's stderr. `run3/`: the Close 4 read on port 9001, the same way.
- `proxy/proxy.jsonl` and `proxy/bodies/` (named by the originals' sha256): the proxy's log, with one desk word in its event names replaced (see `substitutions.txt`). The two mailing answers of 30,125,084 B and 37,647,244 B are kept on the lab PC.
- `reads/`: the raw MARKS and send answers, with their headers; `answers-creates/`, `answers/a3/`: the creates' and the first alteration's answers; `debt/`: the drain record copies.
- `evidence/`: `notes.txt` (each step), the creates and alteration logs, `restore-plan.txt`, the app start and close hash outputs, and the three workbooks with their dumps (`w1/` to `w3/`).
- `scripts/`: the helpers of this sitting, as run.
- `kept-local.txt`: originals of changed files and the files that stay on the lab PC (screens, listings, the backup, the bundle, the generated requests), by sha256 and bytes. `substitutions.txt`: the replacements and their counts. `SHA256.txt`: every file here.
