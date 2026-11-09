First line: **Part W skipped (W1 to W3 and O): it needs the row's desktop app installed, which needs the owner's hands. P0 to A3 ran as EXPECT, and BRIDGE BATCH LAB was then restored by C6 to 224/87, byte-equal to P0's read.**

Sheet `lab/2074-2214-export-cut` at `7a49b5c1`, kit 11 of 11 OK at the start and at the close. Server: `bridge_mcp.exe` 0.7.0 from the 8 Nov bundle (head `1f5dc024`), sha256 `58860204d114ebb0cdab106f35d3753b9f42d36d7bdf10b9feb0f25bac4fecd6`, unchanged at the close. TallyPrime 7.2 Silver. Book BRIDGE BATCH LAB, synthetic. All times are UTC on 9 Nov 2026. Every number is read from its file here, and `evidence/notes.txt` has each step.

## Why Part W was skipped
The desktop app on the lab PC is not the row's artifact. Its `bridge.exe` is FileVersion 0.6.1, 36,892,672 B, written 30 Oct 2026, as are all the files in its folder. The 8 Nov NSIS setup holds `bridge.exe` at 37,611,520 B, so its run did not replace the installed app. An install started from this session runs under the packaged client and lands in a virtualized folder (row 2074-L). So the app needs the owner's hands to install, and steps that need the owner's hands are skipped, as the run lead's queue says. W1, W2, W3 and Part O were not run. The finding also corrected the W4 answers (`lab/2074-2214-w4-answers`, `a16b7cc7`).

## What ran
| step | result |
|---|---|
| P0 | BRIDGE BATCH LAB loaded alone; one `tally.exe` on 9001; `/status` 200; MARKS 224/87, 3,394 B, sha256 `566be20d...` |
| S0 | backup by C5: one file, 57,180 B, sha256 `19031014dde85f88ca6ec7e94315cac627cc2e27955127d576edda40a33edccf`; MARKS after 224/87, byte-equal to P0's |
| list_companies | BRIDGE BATCH LAB alone, `verified_tuple` |
| R1 | STANDARD twice: `total` 11 both; master posts held 31/28 ms and 33/31 ms; post threshold 2,033 ms; drain record clear |
| A1 | `create-probe` CREATED 1 (M 225); `create-000` CREATED 1 (M 226) |
| A2 | MAILING: `total` 13; `Lab Mail Load 000` five entries, 1 to 5 April 2026, oldest first; drain clear |
| A3 | 4,174 alterations, 04:58:24Z to 05:39:08Z, 2,444.3 s; every answer ALTERED 1, every other counter 0, no LINEERROR; last MARKS M 4400, V 87 |

A3's rate was slowed between about 05:03Z and 05:10Z by a disk search the lab ran in the same window (about 1.8 to 1.4 alterations a second). The search was stopped at 05:10Z. A3's count and marks stand; its elapsed time includes that window.

## Close
1. The server was stopped by its stop file (exit code 0) and the relay was stopped. `relay.jsonl` parses whole: 413 lines, 136 forwarded requests, each with its first byte.
2. The kit (11 of 11) and the exe were re-hashed, and both are unchanged.
3. **Restore by C6** of the S0 backup, on the run lead's queue. The first try was refused by Tally at 0%, because the company was open ("Shut the Company and try again"). Nothing was replaced. The company was shut, and the second try answered Accept Yes and Replace only.
4. MARKS after the restore: 224/87, 3,394 B, sha256 `566be20d...`, byte-equal to P0's.
5. C10: company shut; Tally at Select Company with none loaded; one `tally.exe` with 9001 LISTENING; `/status` 200. No XML request after the shut.

Grades: P0 to A3 and the close are **verified**. W1 to W3 and O are **not run**.
